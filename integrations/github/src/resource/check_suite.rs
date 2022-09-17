use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::id;
use crate::resource::{
    App, CheckRunConclusion, CheckRunStatus, GitRef, GitSha, NodeId, PullRequest,
};

id!(
    /// Check suite id
    ///
    /// The [`CheckSuiteId`] is a unique, numerical id that is used to interact with a check suite
    /// through [GitHub's REST API](https://docs.github.com/en/rest).
    CheckSuiteId
);

/// Check suite
///
/// When someone pushes code to a repository, GitHub creates a check suite for the last commit. A
/// check suite is a collection of the check runs created by a single GitHub App for a specific
/// commit. Check suites summarize the status and conclusion of the check runs that a suite
/// includes.
///
/// Read more: https://docs.github.com/en/rest/guides/getting-started-with-the-checks-api
#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct CheckSuite {
    id: CheckSuiteId,
    node_id: NodeId,
    head_branch: GitRef,
    head_sha: GitSha,
    status: CheckRunStatus,
    conclusion: Option<CheckRunConclusion>,
    url: Url,
    before: GitSha,
    after: GitSha,
    pull_requests: Vec<PullRequest>,
    app: App,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl CheckSuite {
    /// Returns the check suite's id.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn id(&self) -> CheckSuiteId {
        self.id
    }

    /// Returns the check suite's node id.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn node_id(&self) -> &NodeId {
        &self.node_id
    }

    /// Returns the check suite's head branch.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn head_branch(&self) -> &GitRef {
        &self.head_branch
    }

    /// Returns the check suite's head SHA.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn head_sha(&self) -> &GitSha {
        &self.head_sha
    }

    /// Returns the check suite's status
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn status(&self) -> CheckRunStatus {
        self.status
    }

    /// Returns the check suite's conclusion.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn conclusion(&self) -> Option<CheckRunConclusion> {
        self.conclusion
    }

    /// Returns the API endpoint to query the check suite.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Returns the check suite's parent commit.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn before(&self) -> &GitSha {
        &self.before
    }

    /// Returns the check suite's head commit.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn after(&self) -> &GitSha {
        &self.after
    }

    /// Returns the check suite's pull requests.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn pull_requests(&self) -> &Vec<PullRequest> {
        &self.pull_requests
    }

    /// Returns the check suite's app.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn app(&self) -> &App {
        &self.app
    }

    /// Returns the date when the check suite was created.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    /// Returns the date when the check suite was last updated.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

impl Display for CheckSuite {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::CheckSuite;

    #[test]
    fn trait_deserialize() {
        let suite: CheckSuite = serde_json::from_str(include_str!(
            "../../tests/fixtures/resource/check_suite.json"
        ))
        .unwrap();

        assert_eq!(7663255123, suite.id().get());
    }

    #[test]
    fn trait_display() {
        let suite: CheckSuite = serde_json::from_str(include_str!(
            "../../tests/fixtures/resource/check_suite.json"
        ))
        .unwrap();

        assert_eq!("7663255123", suite.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<CheckSuite>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<CheckSuite>();
    }
}
