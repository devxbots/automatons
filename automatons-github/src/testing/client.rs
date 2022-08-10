use crate::client::{GitHubClient, PrivateKey};
use crate::resource::{AppId, InstallationId};

pub fn github_client() -> GitHubClient {
    GitHubClient::new(
        mockito::server_url().into(),
        AppId::new(1),
        PrivateKey::new(include_str!("../../tests/fixtures/private-key.pem")),
        InstallationId::new(1),
    )
}
