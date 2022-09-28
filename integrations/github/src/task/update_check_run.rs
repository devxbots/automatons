use anyhow::Context;
use chrono::{DateTime, Utc};
use serde::Serialize;
use url::Url;

use automatons::Error;

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
/// https://docs.github.com/en/rest/checks/runs#update-a-check-run
#[derive(Copy, Clone, Debug)]
pub struct UpdateCheckRun<'a> {
    github_client: &'a GitHubClient,
    owner: &'a Login,
    repository: &'a RepositoryName,
    check_run_args: &'a UpdateCheckRunArgs,
}

/// Input for update check run task
///
/// The input for the task that updates a check run represents the different parameters that
/// GitHub's API accepts.
///
/// https://docs.github.com/en/rest/checks/runs#update-a-check-run
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize)]
pub struct UpdateCheckRunArgs {
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

impl<'a> UpdateCheckRun<'a> {
    /// Initializes the task
    pub fn new(
        github_client: &'a GitHubClient,
        owner: &'a Login,
        repository: &'a RepositoryName,
        check_run_input: &'a UpdateCheckRunArgs,
    ) -> Self {
        Self {
            github_client,
            owner,
            repository,
            check_run_args: check_run_input,
        }
    }

    /// Update a check run
    ///
    /// Updates a check run for a specific commit in a repository.
    pub async fn execute(&self) -> Result<CheckRun, Error> {
        let url = format!(
            "/repos/{}/{}/check-runs/{}",
            self.owner.get(),
            self.repository.get(),
            self.check_run_args.check_run_id
        );

        let check_run = self
            .github_client
            .patch(&url, Some(self.check_run_args))
            .await
            .context("failed to update check run")?;

        Ok(check_run)
    }
}

#[cfg(test)]
mod tests {
    use crate::resource::{CheckRunId, CheckRunName, Login, RepositoryName};
    use crate::testing::check_run::mock_update_check_run;
    use crate::testing::client::github_client;
    use crate::testing::token::mock_installation_access_tokens;

    use super::{UpdateCheckRun, UpdateCheckRunArgs};

    fn input() -> UpdateCheckRunArgs {
        UpdateCheckRunArgs {
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
    async fn task_returns_updated_check_run() {
        let _token_mock = mock_installation_access_tokens();
        let _content_mock = mock_update_check_run();

        let github_client = github_client();
        let login = Login::new("github");
        let repository = RepositoryName::new("hello-world");
        let input = input();

        let task = UpdateCheckRun::new(&github_client, &login, &repository, &input);

        let check_run = task.execute().await.unwrap();

        assert_eq!("mighty_readme", check_run.name().get());
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
