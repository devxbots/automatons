use anyhow::Context;
use reqwest::Method;

use automatons::Error;
use futures::future::try_join_all;

use crate::client::GitHubClient;
use crate::resource::{CheckRun, CheckSuite, GitSha, Login, RepositoryName};

/// List the check runs for a Git reference
///
/// Lists check runs for a commit ref. GitHub Apps must have the `checks:read` permission on a
/// private repository or pull access to a public repository to get check runs. OAuth Apps and
/// authenticated users must have the `repo` scope to get check runs in a private repository.
///
/// https://docs.github.com/en/rest/checks/runs#list-check-runs-in-a-check-suite
#[derive(Copy, Clone, Debug)]
pub struct ListCheckRunsForGitSha<'a> {
    github_client: &'a GitHubClient,
    owner: &'a Login,
    repository: &'a RepositoryName,
    git_sha: &'a GitSha,
}

impl<'a> ListCheckRunsForGitSha<'a> {
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

    /// List the check runs for a Git reference
    ///
    /// Lists check runs for a commit ref.
    pub async fn execute(&self) -> Result<Vec<CheckRun>, Error> {
        let check_suites = self.list_check_suites().await?;
        let check_runs = self.list_check_runs_for_check_suites(&check_suites).await?;

        Ok(check_runs)
    }

    async fn list_check_suites(&self) -> Result<Vec<CheckSuite>, Error> {
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

    async fn list_check_runs_for_check_suites(
        &self,
        check_suites: &[CheckSuite],
    ) -> Result<Vec<CheckRun>, Error> {
        let all_checks_run: Vec<Vec<CheckRun>> = try_join_all(
            check_suites
                .iter()
                .map(|check_suite| self.list_check_runs_for_check_suite(check_suite)),
        )
        .await
        .context("failed to get list of check runs for check suite")?;

        let check_runs: Vec<CheckRun> = all_checks_run.into_iter().flatten().collect();

        Ok(check_runs)
    }

    async fn list_check_runs_for_check_suite(
        &self,
        check_suite: &CheckSuite,
    ) -> Result<Vec<CheckRun>, Error> {
        let url = format!(
            "/repos/{}/{}/check-suites/{}/check-runs",
            self.owner.get(),
            self.repository.get(),
            check_suite.id()
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
    use crate::resource::{GitSha, Login, RepositoryName};
    use crate::testing::check_run::mock_list_check_runs_for_check_suite;
    use crate::testing::check_suite::mock_list_check_suites;
    use crate::testing::client::github_client;
    use crate::testing::token::mock_installation_access_tokens;

    use super::ListCheckRunsForGitSha;

    #[tokio::test]
    async fn task_returns_check_runs() {
        let _token_mock = mock_installation_access_tokens();
        let _check_suite_mock = mock_list_check_suites();
        let _check_runs_mock = mock_list_check_runs_for_check_suite();

        let github_client = github_client();
        let login = Login::new("github");
        let repository = RepositoryName::new("hello-world");
        let git_sha = GitSha::new("d6fde92930d4715a2b49857d24b940956b26d2d3");

        let task = ListCheckRunsForGitSha::new(&github_client, &login, &repository, &git_sha);

        let check_runs = task.execute().await.unwrap();

        assert_eq!(1, check_runs.len());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<ListCheckRunsForGitSha>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<ListCheckRunsForGitSha>();
    }
}
