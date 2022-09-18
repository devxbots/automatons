use anyhow::Context;
use async_trait::async_trait;
use reqwest::Method;

use automatons::{Error, State, Task, Transition};

use crate::client::GitHubClient;
use crate::resource::{CheckRun, CheckSuiteId, Login, RepositoryName};

/// List the check runs for a check suite
///
/// Lists check runs for a check suite using its `id`. GitHub Apps must have the `checks:read`
/// permission on a private repository or pull access to a public repository to get check runs.
/// OAuth Apps and authenticated users must have the `repo` scope to get check runs in a private
/// repository.
///
/// # Parameters
///
/// The task requires the following parameters in the state:
///
/// - `Owner`: The account owner of the repository
/// - `RepositoryName`: The name of the repository
/// - `CheckSuiteId`: The id of the check suite
///
/// https://docs.github.com/en/rest/checks/runs#list-check-runs-in-a-check-suite
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ListCheckRunsForCheckSuite;

#[async_trait]
impl Task for ListCheckRunsForCheckSuite {
    async fn execute(&mut self, state: &mut State) -> Result<Transition, Error> {
        let mut task = TaskImpl::from_state(state)?;

        let check_runs = task.execute(state).await?;
        state.insert(check_runs);

        Ok(Transition::Next)
    }
}

#[derive(Clone, Debug)]
struct TaskImpl<'a> {
    github_client: &'a GitHubClient,
    owner: &'a Login,
    repository: &'a RepositoryName,
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

        Ok(Self {
            github_client,
            owner,
            repository,
        })
    }

    async fn execute(&mut self, state: &State) -> Result<Vec<CheckRun>, Error> {
        let check_suite_id = state
            .get::<CheckSuiteId>()
            .context("failed to get check suite id from state")?;

        let url = format!(
            "/repos/{}/{}/check-suites/{}/check-runs",
            self.owner.get(),
            self.repository.get(),
            check_suite_id
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

    use crate::resource::{CheckRun, CheckSuiteId, Login, RepositoryName};
    use crate::testing::check_run::mock_list_check_runs_for_check_suite;
    use crate::testing::client::github_client;
    use crate::testing::token::mock_installation_access_tokens;

    use super::ListCheckRunsForCheckSuite;

    #[tokio::test]
    async fn task_returns_error_when_github_client_is_missing() {
        let mut state = State::new();

        state.insert(Login::new("github"));
        state.insert(RepositoryName::new("hello-world"));
        state.insert(CheckSuiteId::new(5));

        let mut task = ListCheckRunsForCheckSuite;
        let transition = task.execute(&mut state).await;

        assert!(transition.is_err());
    }

    #[tokio::test]
    async fn task_returns_error_when_login_is_missing() {
        let mut state = State::new();

        state.insert(github_client());
        state.insert(RepositoryName::new("hello-world"));
        state.insert(CheckSuiteId::new(5));

        let mut task = ListCheckRunsForCheckSuite;
        let transition = task.execute(&mut state).await;

        assert!(transition.is_err());
    }
    #[tokio::test]
    async fn task_returns_error_when_repository_name_is_missing() {
        let mut state = State::new();

        state.insert(github_client());
        state.insert(Login::new("github"));
        state.insert(CheckSuiteId::new(5));

        let mut task = ListCheckRunsForCheckSuite;
        let transition = task.execute(&mut state).await;

        assert!(transition.is_err());
    }

    #[tokio::test]
    async fn task_returns_error_when_check_suite_id_is_missing() {
        let mut state = State::new();

        state.insert(github_client());
        state.insert(Login::new("github"));
        state.insert(RepositoryName::new("hello-world"));

        let mut task = ListCheckRunsForCheckSuite;
        let transition = task.execute(&mut state).await;

        assert!(transition.is_err());
    }

    #[tokio::test]
    async fn task_puts_check_runs_into_state() {
        let _token_mock = mock_installation_access_tokens();
        let _content_mock = mock_list_check_runs_for_check_suite();

        let mut state = State::new();

        state.insert(github_client());
        state.insert(Login::new("github"));
        state.insert(RepositoryName::new("hello-world"));
        state.insert(CheckSuiteId::new(5));

        let mut task = ListCheckRunsForCheckSuite;
        let transition = task.execute(&mut state).await.unwrap();

        assert!(matches!(transition, Transition::Next));
        assert!(state.get::<Vec<CheckRun>>().is_some());
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
