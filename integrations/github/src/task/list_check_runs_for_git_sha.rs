use anyhow::Context;
use async_trait::async_trait;
use futures::future::try_join_all;
use reqwest::Method;

use automatons::{Error, State, Task, Transition};

use crate::client::GitHubClient;
use crate::resource::{CheckRun, CheckSuite, GitSha, Login, RepositoryName};

/// List the check runs for a Git reference
///
/// Lists check runs for a commit ref. GitHub Apps must have the `checks:read` permission on a
/// private repository or pull access to a public repository to get check runs. OAuth Apps and
/// authenticated users must have the `repo` scope to get check runs in a private repository.
///
/// # Parameters
///
/// The task requires the following parameters in the state:
///
/// - `Owner`: The account owner of the repository
/// - `RepositoryName`: The name of the repository
/// - `GitSha`: The SHA of the Git commit
///
/// https://docs.github.com/en/rest/checks/runs#list-check-runs-in-a-check-suite
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ListCheckRunsForGitSha;

#[async_trait]
impl Task for ListCheckRunsForGitSha {
    async fn execute(&mut self, state: &mut State) -> Result<Transition, Error> {
        let mut task = TaskImpl::from_state(state)?;

        let check_suites = task.list_check_suites().await?;
        let check_runs = task.list_check_runs_for_check_suites(&check_suites).await?;

        state.insert(check_runs);

        Ok(Transition::Next)
    }
}

#[derive(Clone, Debug)]
struct TaskImpl<'a> {
    github_client: &'a GitHubClient,
    owner: &'a Login,
    repository: &'a RepositoryName,
    git_sha: &'a GitSha,
}

impl<'a> TaskImpl<'a> {
    fn from_state(state: &'a State) -> Result<TaskImpl<'a>, Error> {
        let github_client = state
            .get()
            .context("failed to get GitHub client from state")?;
        let owner = state
            .get::<Login>()
            .context("failed to get owner from state")?;
        let repository = state
            .get::<RepositoryName>()
            .context("failed to get repository name from state")?;
        let git_sha = state
            .get::<GitSha>()
            .context("failed to get Git SHA from state")?;

        Ok(Self {
            github_client,
            owner,
            repository,
            git_sha,
        })
    }

    async fn list_check_suites(&mut self) -> Result<Vec<CheckSuite>, Error> {
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
    use automatons::{State, Task, Transition};

    use crate::resource::{CheckRun, GitSha, Login, RepositoryName};
    use crate::testing::check_run::mock_list_check_runs_for_check_suite;
    use crate::testing::check_suite::mock_list_check_suites;
    use crate::testing::client::github_client;
    use crate::testing::token::mock_installation_access_tokens;

    use super::ListCheckRunsForGitSha;

    #[tokio::test]
    async fn task_returns_error_when_github_client_is_missing() {
        let mut state = State::new();

        state.insert(Login::new("github"));
        state.insert(RepositoryName::new("hello-world"));
        state.insert(GitSha::new("d6fde92930d4715a2b49857d24b940956b26d2d3"));

        let mut task = ListCheckRunsForGitSha;
        let transition = task.execute(&mut state).await;

        assert!(transition.is_err());
    }

    #[tokio::test]
    async fn task_returns_error_when_login_is_missing() {
        let mut state = State::new();

        state.insert(github_client());
        state.insert(RepositoryName::new("hello-world"));
        state.insert(GitSha::new("d6fde92930d4715a2b49857d24b940956b26d2d3"));

        let mut task = ListCheckRunsForGitSha;
        let transition = task.execute(&mut state).await;

        assert!(transition.is_err());
    }
    #[tokio::test]
    async fn task_returns_error_when_repository_name_is_missing() {
        let mut state = State::new();

        state.insert(github_client());
        state.insert(Login::new("github"));
        state.insert(GitSha::new("d6fde92930d4715a2b49857d24b940956b26d2d3"));

        let mut task = ListCheckRunsForGitSha;
        let transition = task.execute(&mut state).await;

        assert!(transition.is_err());
    }

    #[tokio::test]
    async fn task_returns_error_when_git_sha_is_missing() {
        let mut state = State::new();

        state.insert(github_client());
        state.insert(Login::new("github"));
        state.insert(RepositoryName::new("hello-world"));

        let mut task = ListCheckRunsForGitSha;
        let transition = task.execute(&mut state).await;

        assert!(transition.is_err());
    }

    #[tokio::test]
    async fn task_puts_check_runs_into_state() {
        let _token_mock = mock_installation_access_tokens();
        let _check_suite_mock = mock_list_check_suites();
        let _check_runs_mock = mock_list_check_runs_for_check_suite();

        let mut state = State::new();

        state.insert(github_client());
        state.insert(Login::new("github"));
        state.insert(RepositoryName::new("hello-world"));
        state.insert(GitSha::new("d6fde92930d4715a2b49857d24b940956b26d2d3"));

        let mut task = ListCheckRunsForGitSha;
        let transition = task.execute(&mut state).await.unwrap();

        assert!(matches!(transition, Transition::Next));
        assert!(state.get::<Vec<CheckRun>>().is_some());
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
