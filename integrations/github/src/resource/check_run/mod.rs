use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use url::Url;

use crate::resource::{App, CheckSuite, GitSha, NodeId, PullRequest};
use crate::{id, name};

pub use self::conclusion::CheckRunConclusion;
pub use self::output::{CheckRunOutput, CheckRunOutputSummary, CheckRunOutputTitle};
pub use self::status::CheckRunStatus;

mod conclusion;
mod output;
mod status;

id!(
    /// Check run id
    ///
    /// The [`CheckRunId`] is a unique, numerical id that is used to interact with a check run
    /// through [GitHub's REST API](https://docs.github.com/en/rest).
    CheckRunId
);

name!(
    /// Check run name
    ///
    /// Check runs have a human-readable name that is used prominently in GitHub's user interface.
    CheckRunName
);

/// Check run
///
/// A check run is an individual test that is part of a check suite. Each run includes a status and
/// conclusion.
#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct CheckRun {
    id: CheckRunId,
    node_id: NodeId,
    name: CheckRunName,
    head_sha: GitSha,
    external_id: String,
    url: Url,
    html_url: Url,
    details_url: Url,
    status: CheckRunStatus,
    conclusion: Option<CheckRunConclusion>,
    started_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
    check_suite: CheckSuite,
    app: App,
    pull_requests: Vec<PullRequest>,

    #[serde(deserialize_with = "deserialize_output")]
    output: Option<CheckRunOutput>,
}

impl CheckRun {
    /// Returns the check run's id.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn id(&self) -> CheckRunId {
        self.id
    }

    /// Returns the check run's node id.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn node_id(&self) -> &NodeId {
        &self.node_id
    }

    /// Returns the check run's name.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn name(&self) -> &CheckRunName {
        &self.name
    }

    /// Returns the check run's head SHA.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn head_sha(&self) -> &GitSha {
        &self.head_sha
    }

    /// Returns the check run's external id.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn external_id(&self) -> &str {
        &self.external_id
    }

    /// Returns the API endpoint to query the check run.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Returns the URL to the check run.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn html_url(&self) -> &Url {
        &self.html_url
    }

    /// Returns the URL to the check run's details.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn details_url(&self) -> &Url {
        &self.details_url
    }

    /// Returns the check run's status.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn status(&self) -> CheckRunStatus {
        self.status
    }

    /// Returns the check run's conclusion.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn conclusion(&self) -> Option<CheckRunConclusion> {
        self.conclusion
    }

    /// Returns the date when the check run was started.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn started_at(&self) -> &DateTime<Utc> {
        &self.started_at
    }

    /// Returns the date when the check run was completed.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn completed_at(&self) -> &Option<DateTime<Utc>> {
        &self.completed_at
    }

    /// Returns the check run's output.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn output(&self) -> &Option<CheckRunOutput> {
        &self.output
    }

    /// Returns the check run's check suite.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn check_suite(&self) -> &CheckSuite {
        &self.check_suite
    }

    /// Returns the check run's app.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn app(&self) -> &App {
        &self.app
    }

    /// Returns the check run's pull requests.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn pull_requests(&self) -> &Vec<PullRequest> {
        &self.pull_requests
    }
}

impl Display for CheckRun {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn deserialize_output<'de, D>(deserializer: D) -> Result<Option<CheckRunOutput>, D::Error>
where
    D: Deserializer<'de>,
{
    let json = serde_json::Value::deserialize(deserializer)?;

    let title_field = match json.get("title") {
        Some(title) => title,
        None => return Ok(None),
    };

    // Title is a required field, and if it is null the output has not yet been created.
    if title_field.is_null() {
        return Ok(None);
    }

    // TODO: Remove `expect` and return proper error
    let output = serde_json::from_value(json).expect("failed to deserialize check run output");

    Ok(Some(output))
}

#[cfg(test)]
mod tests {
    use super::CheckRun;

    #[test]
    fn trait_deserialize() {
        let check_run: CheckRun = serde_json::from_str(include_str!(
            "../../../tests/fixtures/resource/check_run.json"
        ))
        .unwrap();

        assert_eq!(&None, check_run.output());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<CheckRun>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<CheckRun>();
    }
}
