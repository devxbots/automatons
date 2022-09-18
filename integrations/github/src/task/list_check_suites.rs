use anyhow::Context;
use reqwest::Method;

use automatons::Error;

use crate::client::GitHubClient;
use crate::resource::{CheckSuite, GitSha, Login, RepositoryName};

/// List the check suites for a Git reference
///
/// Lists check suites for a commit `ref`. GitHub Apps must have the `checks:read` permission on a
/// private repository or pull access to a public repository to list check suites. OAuth Apps and
/// authenticated users must have the `repo` scope to get check suites in a private repository.
///
/// https://docs.github.com/en/rest/checks/suites#list-check-suites-for-a-git-reference
#[derive(Copy, Clone, Debug)]
pub struct ListCheckSuites<'a> {
    github_client: &'a GitHubClient,
    owner: &'a Login,
    repository: &'a RepositoryName,
    git_sha: &'a GitSha,
}

impl<'a> ListCheckSuites<'a> {
    /// Initializes the task
    pub fn new(
        github_client: &'a GitHubClient,
        owner: &'a Login,
        repository: &'a RepositoryName,
        git_sha: &'a GitSha,
    ) -> Self {
        Self {
            github_client,
            owner,
            repository,
            git_sha,
        }
    }

    /// List the check suites for a Git reference
    ///
    /// Lists check suites for a commit `ref`.
    pub async fn execute(&self) -> Result<Vec<CheckSuite>, Error> {
        let url = format!(
            "/repos/{}/{}/commits/{}/check-suites",
            self.owner.get(),
            self.repository.get(),
            self.git_sha
        );

        let check_suites = self
            .github_client
            .paginate(Method::GET, &url, "check_suites")
            .await
            .context("failed to query check suites")?;

        Ok(check_suites)
    }
}

#[cfg(test)]
mod tests {
    use crate::resource::{GitSha, Login, RepositoryName};
    use crate::testing::check_suite::mock_list_check_suites;
    use crate::testing::client::github_client;
    use crate::testing::token::mock_installation_access_tokens;

    use super::ListCheckSuites;

    #[tokio::test]
    async fn task_returns_check_suites() {
        let _token_mock = mock_installation_access_tokens();
        let _content_mock = mock_list_check_suites();

        let github_client = github_client();
        let login = Login::new("github");
        let repository = RepositoryName::new("hello-world");
        let git_sha = GitSha::new("d6fde92930d4715a2b49857d24b940956b26d2d3");

        let task = ListCheckSuites::new(&github_client, &login, &repository, &git_sha);

        let check_suites = task.execute().await.unwrap();

        assert_eq!(1, check_suites.len());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<ListCheckSuites>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<ListCheckSuites>();
    }
}
