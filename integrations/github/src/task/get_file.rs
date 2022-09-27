use anyhow::Context;
use base64::decode;
use serde::Deserialize;
use serde_json::Value;
use url::Url;

use automatons::Error;

use crate::client::GitHubClient;
use crate::resource::{File, Login, RepositoryName};

/// Gets a file in a repository
///
/// Gets the contents of a file in a repository.
///
/// # Size limits
///
/// The task only supports files that are smaller than 1MB.
///
/// https://docs.github.com/en/rest/repos/contents#get-repository-content
#[derive(Copy, Clone, Debug)]
pub struct GetFile<'a> {
    github_client: &'a GitHubClient,
    owner: &'a Login,
    repository: &'a RepositoryName,
    path: &'a str,
}

impl<'a> GetFile<'a> {
    /// Initializes the task
    pub fn new(
        github_client: &'a GitHubClient,
        owner: &'a Login,
        repository: &'a RepositoryName,
        path: &'a str,
    ) -> Self {
        Self {
            github_client,
            owner,
            repository,
            path,
        }
    }

    /// Gets a file in a repository
    ///
    /// Gets the contents of a file in a repository.
    pub async fn execute(&self) -> Result<File, Error> {
        let url = format!(
            "/repos/{}/{}/contents/{}",
            self.owner.get(),
            self.repository.get(),
            self.path
        );

        let payload = self.github_client.get(&url).await?;

        let body = match payload {
            GetFileResponse::Success(body) => body,
            GetFileResponse::Error(_) => return Err(Error::NotFound(url)),
        };

        if body.is_array() {
            Err(Error::Serialization(
                "failed to handle unsupported directory payload".into(),
            ))
        } else {
            let payload: GetFilePayload = serde_json::from_value(body).map_err(|_| {
                Error::Serialization(
                    "failed to deserialize payload from GitHub's contents API".into(),
                )
            })?;

            File::try_from(payload)
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Deserialize)]
#[serde(untagged)]
enum GetFileResponse {
    Error(GetFileErrorPayload),
    Success(Value),
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize)]
struct GetFileErrorPayload {
    message: String,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
enum GetFilePayload {
    Directory,
    File(Box<FilePayload>),
    Submodule,
    Symlink,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize)]
struct FilePayload {
    encoding: FileEncoding,
    size: u64,
    name: String,
    path: String,
    content: String,
    sha: String,
    url: Url,
    git_url: Url,
    html_url: Url,
    download_url: Url,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum FileEncoding {
    Base64,
}

impl TryFrom<GetFilePayload> for File {
    type Error = Error;

    fn try_from(value: GetFilePayload) -> Result<Self, Self::Error> {
        let payload = match value {
            GetFilePayload::Directory => Err(Error::Serialization(
                "failed to handle unsupported directory payload".into(),
            )),
            GetFilePayload::File(payload) => Ok(payload),
            GetFilePayload::Submodule => Err(Error::Serialization(
                "failed to handle unsupported submodule payload".into(),
            )),
            GetFilePayload::Symlink => Err(Error::Serialization(
                "failed to handle unsupported symlink payload".into(),
            )),
        }?;

        let sanitized_content = &payload.content.replace('\n', "");
        let content =
            decode(sanitized_content).context("failed to decode Base64 encoded file content")?;

        Ok(File::new(
            payload.name,
            payload.path,
            content,
            payload.sha.into(),
            payload.url,
            payload.git_url,
            payload.html_url,
            payload.download_url,
        ))
    }
}

#[cfg(test)]
mod tests {
    use mockito::mock;

    use automatons::Error;

    use crate::resource::{Login, RepositoryName};
    use crate::testing::client::github_client;
    use crate::testing::contents::{
        mock_get_contents_directory, mock_get_contents_file, mock_get_contents_submodule,
        mock_get_contents_symlink,
    };
    use crate::testing::token::mock_installation_access_tokens;

    use super::GetFile;

    #[tokio::test]
    async fn get_file_with_file() {
        let _token_mock = mock_installation_access_tokens();
        let _content_mock = mock_get_contents_file();

        let github_client = github_client();
        let login = Login::new("octokit");
        let repository = RepositoryName::new("octokit.rb");
        let path = "README.md";

        let task = GetFile::new(&github_client, &login, &repository, path);

        let file = task.execute().await.unwrap();

        assert_eq!("README.md", file.name());
    }

    #[tokio::test]
    async fn get_file_with_directory() {
        let _token_mock = mock_installation_access_tokens();
        let _content_mock = mock_get_contents_directory();

        let github_client = github_client();
        let login = Login::new("octokit");
        let repository = RepositoryName::new("octokit.rb");
        let path = "lib/octokit";

        let task = GetFile::new(&github_client, &login, &repository, path);

        let error = task.execute().await.unwrap_err();
        println!("{:?}", error);

        assert!(matches!(error, Error::Serialization(_)));
    }

    #[tokio::test]
    async fn get_file_with_symlink() {
        let _token_mock = mock_installation_access_tokens();
        let _content_mock = mock_get_contents_symlink();

        let github_client = github_client();
        let login = Login::new("octokit");
        let repository = RepositoryName::new("octokit.rb");
        let path = "bin/some-symlink";

        let task = GetFile::new(&github_client, &login, &repository, path);

        let error = task.execute().await.unwrap_err();

        assert!(matches!(error, Error::Serialization(_)));
    }

    #[tokio::test]
    async fn get_file_with_submodule() {
        let _token_mock = mock_installation_access_tokens();
        let _content_mock = mock_get_contents_submodule();

        let github_client = github_client();
        let login = Login::new("jquery");
        let repository = RepositoryName::new("jquery");
        let path = "test/qunit";

        let task = GetFile::new(&github_client, &login, &repository, path);

        let error = task.execute().await.unwrap_err();

        assert!(matches!(error, Error::Serialization(_)));
    }

    #[tokio::test]
    async fn get_file_not_found() {
        let _token_mock = mock_installation_access_tokens();

        let _content_mock = mock("GET", "/repos/devxbots/automatons/contents/README.md")
            .with_status(404)
            .with_body(r#"
                {
                    "message": "Not Found",
                    "documentation_url": "https://docs.github.com/rest/reference/repos#get-repository-content"
                }
            "#).create();

        let github_client = github_client();
        let login = Login::new("devxbots");
        let repository = RepositoryName::new("automatons");
        let path = "README.md";

        let task = GetFile::new(&github_client, &login, &repository, path);

        let error = task.execute().await.unwrap_err();

        assert!(matches!(error, Error::NotFound(_)));
    }
}
