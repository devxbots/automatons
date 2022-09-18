use anyhow::Context;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Serialize;
use url::Url;

use automatons::{Error, State, Task, Transition};

use crate::client::GitHubClient;
use crate::resource::{
    CheckRun, CheckRunConclusion, CheckRunId, CheckRunName, CheckRunOutput, CheckRunStatus, Login,
    RepositoryName,
};

/// Update a check run
///
/// Updates a check run for a specific commit in a repository. The GitHub App must have the
/// `checks:write` permission to edit check runs.
///
/// # Parameters
///
/// The task requires the following parameters in the state:
///
/// - `Owner`: The account owner of the repository
/// - `RepositoryName`: The name of the repository
/// - `UpdateCheckRunInput`: The input for the task
///
/// https://docs.github.com/en/rest/checks/runs#update-a-check-run
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct UpdateCheckRun;

/// Input for update check run task
///
/// The input for the task that updates a check run represents the different parameters that
/// GitHub's API accepts.
///
/// https://docs.github.com/en/rest/checks/runs#update-a-check-run
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize)]
pub struct UpdateCheckRunInput {
    /// The unique identifier of the check run.
    pub check_run_id: CheckRunId,

    /// The name of the check. For example, "code-coverage".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CheckRunName>,

    /// The URL of the integrator's site that has the full details of the check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_url: Option<Url>,

    /// A reference for the run on the integrator's system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,

    /// The time that the check run began.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,

    /// The current status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CheckRunStatus>,

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
impl Task for UpdateCheckRun {
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
    check_run_input: &'a UpdateCheckRunInput,
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
            .get::<UpdateCheckRunInput>()
            .context("failed to get input for update check run task from state")?;

        Ok(Self {
            github_client,
            owner,
            repository,
            check_run_input,
        })
    }

    async fn execute(&self) -> Result<CheckRun, Error> {
        let url = format!(
            "/repos/{}/{}/check-runs/{}",
            self.owner.get(),
            self.repository.get(),
            self.check_run_input.check_run_id
        );

        let check_run = self
            .github_client
            .patch(&url, Some(self.check_run_input))
            .await
            .context("failed to update check run")?;

        Ok(check_run)
    }
}

#[cfg(test)]
mod tests {
    use automatons::{State, Task, Transition};

    use crate::resource::{CheckRun, CheckRunId, CheckRunName, Login, RepositoryName};
    use crate::testing::check_run::mock_update_check_run;
    use crate::testing::client::github_client;
    use crate::testing::token::mock_installation_access_tokens;

    use super::{UpdateCheckRun, UpdateCheckRunInput};

    fn input() -> UpdateCheckRunInput {
        UpdateCheckRunInput {
            check_run_id: CheckRunId::new(4),
            name: Some(CheckRunName::new("mighty_readme")),
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

        let mut task = UpdateCheckRun;
        let transition = task.execute(&mut state).await;

        assert!(transition.is_err());
    }

    #[tokio::test]
    async fn task_returns_error_when_login_is_missing() {
        let mut state = State::new();

        state.insert(github_client());
        state.insert(RepositoryName::new("hello-world"));
        state.insert(input());

        let mut task = UpdateCheckRun;
        let transition = task.execute(&mut state).await;

        assert!(transition.is_err());
    }
    #[tokio::test]
    async fn task_returns_error_when_repository_name_is_missing() {
        let mut state = State::new();

        state.insert(github_client());
        state.insert(Login::new("github"));
        state.insert(input());

        let mut task = UpdateCheckRun;
        let transition = task.execute(&mut state).await;

        assert!(transition.is_err());
    }

    #[tokio::test]
    async fn task_returns_error_when_input_is_missing() {
        let mut state = State::new();

        state.insert(github_client());
        state.insert(Login::new("github"));
        state.insert(RepositoryName::new("hello-world"));

        let mut task = UpdateCheckRun;
        let transition = task.execute(&mut state).await;

        assert!(transition.is_err());
    }

    #[tokio::test]
    async fn task_puts_check_run_into_state() {
        let _token_mock = mock_installation_access_tokens();
        let _content_mock = mock_update_check_run();

        let mut state = State::new();

        state.insert(github_client());
        state.insert(Login::new("github"));
        state.insert(RepositoryName::new("hello-world"));
        state.insert(input());

        let mut task = UpdateCheckRun;
        let transition = task.execute(&mut state).await.unwrap();

        assert!(matches!(transition, Transition::Next));
        assert!(state.get::<CheckRun>().is_some());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<UpdateCheckRun>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<UpdateCheckRun>();
    }
}
