use anyhow::Context;
use reqwest::Method;

use async_trait::async_trait;
use automatons::{Error, State, Task, Transition};

use crate::client::GitHubClient;
use crate::resource::{CheckSuite, GitSha, Login, RepositoryName};

/// List the check suites for a Git reference
///
/// Lists check suites for a commit `ref`. GitHub Apps must have the `checks:read` permission on a
/// private repository or pull access to a public repository to list check suites. OAuth Apps and
/// authenticated users must have the `repo` scope to get check suites in a private repository.
///
/// # Parameters
///
/// The task requires the following parameters in the state:
///
/// - `Owner`: The account owner of the repository
/// - `RepositoryName`: The name of the repository
/// - `GitSha`: The Git commit ref
///
/// https://docs.github.com/en/rest/checks/suites#list-check-suites-for-a-git-reference
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ListCheckSuites;

#[async_trait]
impl Task for ListCheckSuites {
    async fn execute(&mut self, state: &mut State) -> Result<Transition, Error> {
        let mut task = TaskImpl::from_state(state)?;

        let check_suites = task.execute(state).await?;
        state.insert(check_suites);

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
    pub fn from_state(state: &'a State) -> Result<TaskImpl<'a>, Error> {
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

    async fn execute(&mut self, state: &State) -> Result<Vec<CheckSuite>, Error> {
        let head_sha = state
            .get::<GitSha>()
            .context("failed to get check suite id from state")?;

        let url = format!(
            "/repos/{}/{}/commits/{}/check-suites",
            self.owner.get(),
            self.repository.get(),
            head_sha
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
    use automatons::{State, Task, Transition};

    use crate::resource::{CheckSuite, GitSha, Login, RepositoryName};
    use crate::testing::check_suite::mock_list_check_suites;
    use crate::testing::client::github_client;
    use crate::testing::token::mock_installation_access_tokens;

    use super::ListCheckSuites;

    #[tokio::test]
    async fn task_returns_error_when_github_client_is_missing() {
        let mut state = State::new();

        state.insert(Login::new("github"));
        state.insert(RepositoryName::new("hello-world"));
        state.insert(GitSha::new("d6fde92930d4715a2b49857d24b940956b26d2d3"));

        let mut task = ListCheckSuites;
        let transition = task.execute(&mut state).await;

        assert!(transition.is_err());
    }

    #[tokio::test]
    async fn task_returns_error_when_login_is_missing() {
        let mut state = State::new();

        state.insert(github_client());
        state.insert(RepositoryName::new("hello-world"));
        state.insert(GitSha::new("d6fde92930d4715a2b49857d24b940956b26d2d3"));

        let mut task = ListCheckSuites;
        let transition = task.execute(&mut state).await;

        assert!(transition.is_err());
    }
    #[tokio::test]
    async fn task_returns_error_when_repository_name_is_missing() {
        let mut state = State::new();

        state.insert(github_client());
        state.insert(Login::new("github"));
        state.insert(GitSha::new("d6fde92930d4715a2b49857d24b940956b26d2d3"));

        let mut task = ListCheckSuites;
        let transition = task.execute(&mut state).await;

        assert!(transition.is_err());
    }

    #[tokio::test]
    async fn task_returns_error_when_head_sha_is_missing() {
        let mut state = State::new();

        state.insert(github_client());
        state.insert(Login::new("github"));
        state.insert(RepositoryName::new("hello-world"));

        let mut task = ListCheckSuites;
        let transition = task.execute(&mut state).await;

        assert!(transition.is_err());
    }

    #[tokio::test]
    async fn task_puts_check_suites_into_state() {
        let _token_mock = mock_installation_access_tokens();
        let _content_mock = mock_list_check_suites();

        let mut state = State::new();

        state.insert(github_client());
        state.insert(Login::new("github"));
        state.insert(RepositoryName::new("hello-world"));
        state.insert(GitSha::new("d6fde92930d4715a2b49857d24b940956b26d2d3"));

        let mut task = ListCheckSuites;
        let transition = task.execute(&mut state).await.unwrap();

        assert!(matches!(transition, Transition::Next));
        assert!(state.get::<Vec<CheckSuite>>().is_some());
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
