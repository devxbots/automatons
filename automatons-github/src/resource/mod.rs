//! Resources on GitHub
//!
//! The GitHub integration enables the [automatons] framework to interact with resources on GitHub.
//! These resources are modelled as strongly-typed data types in this module, and each model maps to
//! a resource in [GitHub's REST API](https://docs.github.com/en/rest).
//!
//! [automatons]: https://github.com/devxbots/automatons

use crate::name;

pub use self::account::{Account, AccountId, AccountType, Login};
pub use self::app::{App, AppId, AppName, AppSlug};
pub use self::check_run::{CheckRunConclusion, CheckRunStatus};
pub use self::check_suite::CheckSuite;
pub use self::installation::{Installation, InstallationId};
pub use self::license::{License, LicenseKey, LicenseName, SpdxId};
pub use self::organization::{Organization, OrganizationId};
pub use self::pull_request::{PullRequest, PullRequestBranch, PullRequestId, PullRequestNumber};
pub use self::repository::{
    MinimalRepository, Repository, RepositoryFullName, RepositoryId, RepositoryName,
};
pub use self::visibility::Visibility;

mod account;
mod app;
mod check_run;
mod check_suite;
mod installation;
mod license;
mod organization;
mod pull_request;
mod repository;
mod visibility;

name!(
    /// Unique identifier used with GitHub's GraphQL API
    ///
    /// GitHub assigns a unique `node_id` to most resources on the platform, which identifies the
    /// resource in [GitHub's GraphQL API](https://docs.github.com/en/graphql).
    NodeId
);

#[cfg(test)]
mod tests {
    use super::NodeId;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<NodeId>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<NodeId>();
    }
}
