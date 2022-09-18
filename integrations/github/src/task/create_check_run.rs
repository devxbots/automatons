use anyhow::Context;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Serialize;
use url::Url;

use automatons::{Error, State, Task, Transition};

use crate::client::GitHubClient;
use crate::resource::{
    CheckRun, CheckRunConclusion, CheckRunName, CheckRunOutput, CheckRunStatus, GitSha, Login,
    RepositoryName,
};

/// Create a check run
///
/// Creates a new check run for a specific commit in a repository. The GitHub App must have the
/// `checks:write` permission to create check runs.
///
/// In a check suite, GitHub limits the number of check runs with the same name to 1000. Once these
/// check runs exceed 1000, GitHub will start to automatically delete older check runs.
///
/// # Parameters
///
/// The task requires the following parameters in the state:
///
/// - `Owner`: The account owner of the repository
/// - `RepositoryName`: The name of the repository
/// - `CreateCheckRunInput`: The input for the task
///
/// https://docs.github.com/en/rest/checks/runs#create-a-check-run
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct CreateCheckRun;

/// Input for create check run task
///
/// The input for the task that creates a check run represents the different parameters that
/// GitHub's API accepts.
///
/// https://docs.github.com/en/rest/checks/runs#create-a-check-run
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize)]
pub struct CreateCheckRunInput {
    /// The name of the check. For example, "code-coverage".
    pub name: CheckRunName,

    /// The SHA of the commit.
    pub head_sha: GitSha,

    /// The URL of the integrator's site that has the full details of the check. If the integrator
    /// does not provide this, then the homepage of the GitHub app is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_url: Option<Url>,

    /// A reference for the run on the integrator's system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,

    /// The current status. `queued` by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CheckRunStatus>,

    /// The time that the check run began.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,

    /// The final conclusion of the check.
    ///
    /// Required if you provide `completed_at` or a status of `completed`. Providing a conclusion
    /// will automatically set the status parameter to `completed`. You cannot change a check run
    /// conclusion to `stale`, only GitHub can set this.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<CheckRunConclusion>,

    /// The time the check completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,

    /// Check runs can accept a variety of data in the output object, including a title and summary
    /// and can optionally provide descriptive details about the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<CheckRunOutput>,
}

#[async_trait]
impl Task for CreateCheckRun {
    async fn execute(&mut self, state: &mut State) -> Result<Transition, Error> {
        let task = TaskImpl::from_state(state)?;

        let check_run = task.execute().await?;
        state.insert(check_run);

        Ok(Transition::Next)
    }
}

#[derive(Clone, Debug)]
struct TaskImpl<'a> {
    github_client: &'a GitHubClient,
    owner: &'a Login,
    repository: &'a RepositoryName,
    check_run_input: &'a CreateCheckRunInput,
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
        let check_run_input = state
            .get::<CreateCheckRunInput>()
            .context("failed to get input for create check run task from state")?;

        Ok(Self {
            github_client,
            owner,
            repository,
            check_run_input,
        })
    }

    async fn execute(&self) -> Result<CheckRun, Error> {
        let url = format!(
            "/repos/{}/{}/check-runs",
            self.owner.get(),
            self.repository.get(),
        );

        let check_run = self
            .github_client
            .post(&url, Some(self.check_run_input))
            .await
            .context("failed to create check run")?;

        Ok(check_run)
    }
}

#[cfg(test)]
mod tests {
    use automatons::{State, Task, Transition};

    use crate::resource::{CheckRun, CheckRunName, GitSha, Login, RepositoryName};
    use crate::testing::check_run::mock_create_check_run;
    use crate::testing::client::github_client;
    use crate::testing::token::mock_installation_access_tokens;

    use super::{CreateCheckRun, CreateCheckRunInput};

    fn input() -> CreateCheckRunInput {
        CreateCheckRunInput {
            name: CheckRunName::new("mighty_readme"),
            head_sha: GitSha::new("ce587453ced02b1526dfb4cb910479d431683101"),
            details_url: None,
            external_id: None,
            status: None,
            started_at: None,
            conclusion: None,
            completed_at: None,
            output: None,
        }
    }

    #[tokio::test]
    async fn task_returns_error_when_github_client_is_missing() {
        let mut state = State::new();

        state.insert(Login::new("github"));
        state.insert(RepositoryName::new("hello-world"));
        state.insert(input());

        let mut task = CreateCheckRun;
        let transition = task.execute(&mut state).await;

        assert!(transition.is_err());
    }

    #[tokio::test]
    async fn task_returns_error_when_login_is_missing() {
        let mut state = State::new();

        state.insert(github_client());
        state.insert(RepositoryName::new("hello-world"));
        state.insert(input());

        let mut task = CreateCheckRun;
        let transition = task.execute(&mut state).await;

        assert!(transition.is_err());
    }
    #[tokio::test]
    async fn task_returns_error_when_repository_name_is_missing() {
        let mut state = State::new();

        state.insert(github_client());
        state.insert(Login::new("github"));
        state.insert(input());

        let mut task = CreateCheckRun;
        let transition = task.execute(&mut state).await;

        assert!(transition.is_err());
    }

    #[tokio::test]
    async fn task_returns_error_when_input_is_missing() {
        let mut state = State::new();

        state.insert(github_client());
        state.insert(Login::new("github"));
        state.insert(RepositoryName::new("hello-world"));

        let mut task = CreateCheckRun;
        let transition = task.execute(&mut state).await;

        assert!(transition.is_err());
    }

    #[tokio::test]
    async fn task_puts_check_run_into_state() {
        let _token_mock = mock_installation_access_tokens();
        let _content_mock = mock_create_check_run();

        let mut state = State::new();

        state.insert(github_client());
        state.insert(Login::new("github"));
        state.insert(RepositoryName::new("hello-world"));
        state.insert(input());

        let mut task = CreateCheckRun;
        let transition = task.execute(&mut state).await.unwrap();

        assert!(matches!(transition, Transition::Next));
        assert!(state.get::<CheckRun>().is_some());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<CreateCheckRun>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<CreateCheckRun>();
    }
}
