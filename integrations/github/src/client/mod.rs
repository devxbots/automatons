use anyhow::{anyhow, Context};
use reqwest::header::HeaderValue;
use reqwest::{Client, Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use crate::client::error::ClientError;
use crate::resource::{AppId, InstallationId};
use crate::{name, secret};

pub use self::token::{AppScope, InstallationScope, Token, TokenFactory};

mod error;
mod token;

name!(
    /// API endpoint for the client
    ///
    /// The GitHub client can be used with different GitHub instances, for example a self-hosted
    /// GitHub Enterprise Server. The `GitHubHost` sets the base URL that the client will use.
    GitHubHost
);

secret!(
    /// Private key of the GitHub App
    ///
    /// GitHub Apps have a private key that they use to sign authentication tokens.
    PrivateKey
);

/// Client for GitHub's REST API
///
/// The GitHub client can be used to send HTTP requests to GitHub's REST API. The client handles
/// authentication, serialization, and pagination.
#[derive(Clone, Debug)]
pub struct GitHubClient {
    github_host: GitHubHost,
    token_factory: TokenFactory,
    installation_id: InstallationId,
}

#[allow(dead_code)] // TODO: Remove when remaining tasks have been migrated from `github-parts`
impl GitHubClient {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn new(
        github_host: GitHubHost,
        app_id: AppId,
        private_key: PrivateKey,
        installation_id: InstallationId,
    ) -> Self {
        let token_factory = TokenFactory::new(github_host.clone(), app_id, private_key);

        Self {
            github_host,
            token_factory,
            installation_id,
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub async fn get<T>(&self, endpoint: &str) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
    {
        // We need to explicitly declare the type of the body somewhere to silence a compiler error.
        let body: Option<Value> = None;

        self.send_request(Method::GET, endpoint, body).await
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(body)))]
    pub async fn post<T>(
        &self,
        endpoint: &str,
        body: Option<impl Serialize>,
    ) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
    {
        self.send_request(Method::POST, endpoint, body).await
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(body)))]
    pub async fn patch<T>(
        &self,
        endpoint: &str,
        body: Option<impl Serialize>,
    ) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
    {
        self.send_request(Method::PATCH, endpoint, body).await
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(body)))]
    async fn send_request<T>(
        &self,
        method: Method,
        endpoint: &str,
        body: Option<impl Serialize>,
    ) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.github_host.get(), endpoint);

        let mut client = self.client(method.clone(), &url).await?;

        if let Some(body) = body {
            client = client.json(&body);
        }

        let response = client.send().await?;
        let status = &response.status();

        if !status.is_success() {
            #[cfg(feature = "tracing")]
            tracing::error!(
                "failed to send {} request to GitHub: {:?}",
                &method,
                response.text().await?
            );

            return if status == &404 {
                Err(ClientError::NotFound)
            } else {
                // TODO: Gracefully return status instead of error
                Err(ClientError::Unknown(anyhow!(
                    "failed to send {} request to GitHub",
                    &method
                )))
            };
        }

        let data = response.json::<T>().await?;

        Ok(data)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub async fn paginate<T>(
        &self,
        method: Method,
        endpoint: &str,
        key: &str,
    ) -> Result<Vec<T>, ClientError>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.github_host.get(), endpoint);

        let mut collection = Vec::new();
        let mut next_url = Some(url);

        while next_url.is_some() {
            let response = self
                .client(method.clone(), &next_url.unwrap())
                .await?
                .send()
                .await?;

            next_url = self.get_next_url(response.headers().get("link"))?;
            let body = &response.json::<Value>().await?;

            let payload = body
                .get(key)
                .context("failed to find pagination key in HTTP response")?;

            // TODO: Avoid cloning the payload
            let mut entities: Vec<T> = serde_json::from_value(payload.clone())
                .context("failed to deserialize paginated entities")?;

            collection.append(&mut entities);
        }

        Ok(collection)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    async fn client(&self, method: Method, url: &str) -> Result<RequestBuilder, ClientError> {
        let token = self
            .token_factory
            .installation(self.installation_id)
            .await
            .context("failed to get authentication token from factory")?;

        let client = Client::new()
            .request(method, url)
            .header("Authorization", format!("Bearer {}", token.get()))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "devxbots/github-parts");

        Ok(client)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn get_next_url(&self, header: Option<&HeaderValue>) -> Result<Option<String>, ClientError> {
        let header = match header {
            Some(header) => header,
            None => return Ok(None),
        };

        let relations: Vec<&str> = header
            .to_str()
            .context("failed to parse HTTP request header")?
            .split(',')
            .collect();

        let next_rel = match relations.iter().find(|link| link.contains(r#"rel="next"#)) {
            Some(link) => link,
            None => return Ok(None),
        };

        let link_start_position = 1 + next_rel
            .find('<')
            .context("failed to extract next url from link header")?;
        let link_end_position = next_rel
            .find('>')
            .context("failed to extract next url from link header")?;

        let link = String::from(&next_rel[link_start_position..link_end_position]);

        Ok(Some(link))
    }
}

#[cfg(test)]
mod tests {
    use reqwest::header::HeaderValue;
    use reqwest::Method;

    use mockito::mock;

    use crate::client::PrivateKey;
    use crate::resource::{AppId, InstallationId, Repository};

    use super::GitHubClient;

    #[tokio::test]
    async fn get_entity() {
        let _token_mock = mock("POST", "/app/installations/1/access_tokens")
            .with_status(200)
            .with_body(r#"{ "token": "ghs_16C7e42F292c6912E7710c838347Ae178B4a" }"#)
            .create();
        let _content_mock = mock("GET", "/repos/devxbots/automatons")
            .with_status(200)
            .with_body_from_file("tests/fixtures/resource/repository.json")
            .create();

        let client = GitHubClient::new(
            mockito::server_url().into(),
            AppId::new(1),
            PrivateKey::new(include_str!("../../tests/fixtures/private-key.pem")),
            InstallationId::new(1),
        );

        let repository: Repository = client.get("/repos/devxbots/automatons").await.unwrap();

        assert_eq!(518377950, repository.id().get());
    }

    #[tokio::test]
    async fn paginate_returns_all_entities() {
        let _token_mock = mock("POST", "/app/installations/1/access_tokens")
            .with_status(200)
            .with_body(r#"{ "token": "ghs_16C7e42F292c6912E7710c838347Ae178B4a" }"#)
            .create();
        let _first_page_mock = mock("GET", "/installation/repositories")
            .with_status(200)
            .with_header(
                "link",
                &format!(
                    "<{}/installation/repositories?page=2>; rel=\"next\"",
                    mockito::server_url()
                ),
            )
            .with_body(format!(
                r#"
                {{
                    "total_count": 2,
                    "repositories": [
                        {}
                    ]
                }}
                "#,
                include_str!("../../tests/fixtures/resource/repository.json")
            ))
            .create();
        let _second_page_mock = mock("GET", "/installation/repositories?page=2")
            .with_status(200)
            .with_body(format!(
                r#"
                {{
                    "total_count": 2,
                    "repositories": [
                        {}
                    ]
                }}
                "#,
                include_str!("../../tests/fixtures/resource/repository.json")
            ))
            .create();

        let client = GitHubClient::new(
            mockito::server_url().into(),
            AppId::new(1),
            PrivateKey::new(include_str!("../../tests/fixtures/private-key.pem")),
            InstallationId::new(1),
        );

        let repository: Vec<Repository> = client
            .paginate(Method::GET, "/installation/repositories", "repositories")
            .await
            .unwrap();

        assert_eq!(2, repository.len());
    }

    #[test]
    fn get_next_url_returns_url() {
        let client = GitHubClient::new(
            mockito::server_url().into(),
            AppId::new(1),
            PrivateKey::new(include_str!("../../tests/fixtures/private-key.pem")),
            InstallationId::new(1),
        );

        let header = HeaderValue::from_str(r#"<https://api.github.com/search/code?q=addClass+user%3Amozilla&page=13>; rel="prev", <https://api.github.com/search/code?q=addClass+user%3Amozilla&page=15>; rel="next", <https://api.github.com/search/code?q=addClass+user%3Amozilla&page=34>; rel="last", <https://api.github.com/search/code?q=addClass+user%3Amozilla&page=1>; rel="first""#).unwrap();

        let next_url = client.get_next_url(Some(&header)).unwrap().unwrap();

        assert_eq!(
            "https://api.github.com/search/code?q=addClass+user%3Amozilla&page=15",
            next_url
        );
    }

    #[test]
    fn get_next_url_returns_none() {
        let client = GitHubClient::new(
            mockito::server_url().into(),
            AppId::new(1),
            PrivateKey::new(include_str!("../../tests/fixtures/private-key.pem")),
            InstallationId::new(1),
        );

        let header = HeaderValue::from_str(
            r#"<https://api.github.com/search/code?q=addClass+user%3Amozilla&page=13>; rel="prev""#,
        )
        .unwrap();

        let next_url = client.get_next_url(Some(&header)).unwrap();

        assert!(next_url.is_none());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<GitHubClient>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<GitHubClient>();
    }
}
