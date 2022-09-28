use anyhow::Context;
use chrono::{DateTime, Utc};
use serde::Serialize;
use url::Url;

use automatons::Error;

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
/// https://docs.github.com/en/rest/checks/runs#create-a-check-run
#[derive(Copy, Clone, Debug)]
pub struct CreateCheckRun<'a> {
    github_client: &'a GitHubClient,
    owner: &'a Login,
    repository: &'a RepositoryName,
    check_run_args: &'a CreateCheckRunArgs,
}

/// Input for create check run task
///
/// The input for the task that creates a check run represents the different parameters that
/// GitHub's API accepts.
///
/// https://docs.github.com/en/rest/checks/runs#create-a-check-run
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize)]
pub struct CreateCheckRunArgs {
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

impl<'a> CreateCheckRun<'a> {
    /// Initializes the task
    pub fn new(
        github_client: &'a GitHubClient,
        owner: &'a Login,
        repository: &'a RepositoryName,
        check_run_input: &'a CreateCheckRunArgs,
    ) -> Self {
        Self {
            github_client,
            owner,
            repository,
            check_run_args: check_run_input,
        }
    }

    /// Create a check run
    pub async fn execute(&self) -> Result<CheckRun, Error> {
        let url = format!(
            "/repos/{}/{}/check-runs",
            self.owner.get(),
            self.repository.get(),
        );

        let check_run = self
            .github_client
            .post(&url, Some(self.check_run_args))
            .await
            .context("failed to create check run")?;

        Ok(check_run)
    }
}

#[cfg(test)]
mod tests {
    use crate::resource::{CheckRunName, GitSha, Login, RepositoryName};
    use crate::testing::check_run::mock_create_check_run;
    use crate::testing::client::github_client;
    use crate::testing::token::mock_installation_access_tokens;

    use super::{CreateCheckRun, CreateCheckRunArgs};

    fn input() -> CreateCheckRunArgs {
        CreateCheckRunArgs {
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
    async fn task_returns_check_run() {
        let _token_mock = mock_installation_access_tokens();
        let _content_mock = mock_create_check_run();

        let github_client = github_client();
        let login = Login::new("github");
        let repository = RepositoryName::new("hello-world");
        let check_run_input = input();

        let task = CreateCheckRun::new(&github_client, &login, &repository, &check_run_input);

        let check_run = task.execute().await.unwrap();

        assert_eq!(4, check_run.id().get());
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
