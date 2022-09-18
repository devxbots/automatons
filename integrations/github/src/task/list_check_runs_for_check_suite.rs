use anyhow::Context;
use reqwest::Method;

use automatons::Error;

use crate::client::GitHubClient;
use crate::resource::{CheckRun, CheckSuiteId, Login, RepositoryName};

/// List the check runs for a check suite
///
/// Lists check runs for a check suite using its `id`. GitHub Apps must have the `checks:read`
/// permission on a private repository or pull access to a public repository to get check runs.
/// OAuth Apps and authenticated users must have the `repo` scope to get check runs in a private
/// repository.
///
/// https://docs.github.com/en/rest/checks/runs#list-check-runs-in-a-check-suite
#[derive(Copy, Clone, Debug)]
pub struct ListCheckRunsForCheckSuite<'a> {
    github_client: &'a GitHubClient,
    owner: &'a Login,
    repository: &'a RepositoryName,
    check_suite_id: &'a CheckSuiteId,
}

impl<'a> ListCheckRunsForCheckSuite<'a> {
    /// Initializes the task
    pub fn new(
        github_client: &'a GitHubClient,
        owner: &'a Login,
        repository: &'a RepositoryName,
        check_suite_id: &'a CheckSuiteId,
    ) -> Self {
        Self {
            github_client,
            owner,
            repository,
            check_suite_id,
        }
    }

    /// List the check runs for a check suite
    ///
    /// Lists check runs for a check suite using its `id`.
    pub async fn execute(&self) -> Result<Vec<CheckRun>, Error> {
        let url = format!(
            "/repos/{}/{}/check-suites/{}/check-runs",
            self.owner.get(),
            self.repository.get(),
            self.check_suite_id
        );

        let check_runs = self
            .github_client
            .paginate(Method::GET, &url, "check_runs")
            .await
            .context("failed to query check runs")?;

        Ok(check_runs)
    }
}

#[cfg(test)]
mod tests {
    use crate::resource::{CheckSuiteId, Login, RepositoryName};
    use crate::testing::check_run::mock_list_check_runs_for_check_suite;
    use crate::testing::client::github_client;
    use crate::testing::token::mock_installation_access_tokens;

    use super::ListCheckRunsForCheckSuite;

    #[tokio::test]
    async fn task_returns_check_runs() {
        let _token_mock = mock_installation_access_tokens();
        let _content_mock = mock_list_check_runs_for_check_suite();

        let github_client = github_client();
        let login = Login::new("github");
        let repository = RepositoryName::new("hello-world");
        let check_suite_id = CheckSuiteId::new(5);

        let task =
            ListCheckRunsForCheckSuite::new(&github_client, &login, &repository, &check_suite_id);

        let check_runs = task.execute().await.unwrap();

        assert_eq!(1, check_runs.len());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<ListCheckRunsForCheckSuite>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<ListCheckRunsForCheckSuite>();
    }
}
