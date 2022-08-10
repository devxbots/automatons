use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use url::Url;

use crate::id;

pub use self::branch::PullRequestBranch;

mod branch;

id!(
    /// Pull request id
    ///
    /// The [`PullRequestId`] is a unique, numerical id that is used to interact with a pull request
    /// through [GitHub's REST API](https://docs.github.com/en/rest).
    PullRequestId
);

id!(
    /// Pull request number
    ///
    /// Every [`PullRequest`] has a unique, human-readable, monotonically increasing number assigned
    /// to it. This number identifies the pull request on GitHub's website.
    PullRequestNumber
);

/// Pull request
///
/// Pull requests are a feature of GitHub to merge two branches. Users can create, review, and merge
/// pull requests using GitHub's platform. Each pull request has a unique `id`, a human-readable
/// `number`, and references to the two branches.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct PullRequest {
    id: PullRequestId,
    number: PullRequestNumber,
    url: Url,
    head: PullRequestBranch,
    base: PullRequestBranch,
}

impl PullRequest {
    /// Returns the pull request's id.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn id(&self) -> PullRequestId {
        self.id
    }

    /// Returns the pull request's number.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn number(&self) -> PullRequestNumber {
        self.number
    }

    /// Returns the API endpoint to query the pull request.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Returns the pull request's head branch
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn head(&self) -> &PullRequestBranch {
        &self.head
    }

    /// Returns the pull request's base branch
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn base(&self) -> &PullRequestBranch {
        &self.base
    }
}

impl Display for PullRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.number)
    }
}

#[cfg(test)]
mod tests {
    use super::PullRequest;

    #[test]
    fn trait_deserialize() {
        let pr: PullRequest = serde_json::from_str(include_str!(
            "../../../tests/fixtures/resource/pull_request.json"
        ))
        .unwrap();

        assert_eq!(27, pr.number().get());
    }

    #[test]
    fn trait_display() {
        let pr: PullRequest = serde_json::from_str(include_str!(
            "../../../tests/fixtures/resource/pull_request.json"
        ))
        .unwrap();

        assert_eq!("#27", pr.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<PullRequest>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<PullRequest>();
    }
}
