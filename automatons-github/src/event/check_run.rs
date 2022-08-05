use std::fmt::{Display, Formatter};

use crate::resource::{Account, CheckRun, Installation, Organization, Repository};

/// Check run action
///
/// The type of activity that has occurred.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum CheckRunAction {
    /// A new check run was created.
    Created,

    /// The `status` of the [`CheckRun`] is `completed`.
    Completed,

    /// Someone requested to re-run your check run from the pull request UI.
    Rerequested,

    /// Someone requested an action your app provides to be taken.
    RequestedAction,
}

/// Check run event
///
/// A check run event contains the action that occurred, the latest state of the check run, and the
/// repository that the check run was created in. If the webhook was configured for an organization,
/// or if the repository is owned by one, the organization is included in the payload. If the event
/// is sent to a GitHub App, the payload contains the installation.
#[derive(Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct CheckRunEvent {
    action: CheckRunAction,
    check_run: CheckRun,
    repository: Repository,
    organization: Option<Organization>,
    installation: Option<Installation>,
    sender: Account,
}

impl CheckRunEvent {
    /// Returns the check run event's action.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn action(&self) -> CheckRunAction {
        self.action
    }

    /// Returns the check run event's check run.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn check_run(&self) -> &CheckRun {
        &self.check_run
    }

    /// Returns the check run event's repository.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn repository(&self) -> &Repository {
        &self.repository
    }

    /// Returns the check run event's organization.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn organization(&self) -> &Option<Organization> {
        &self.organization
    }

    /// Returns the check run event's installation.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn installation(&self) -> &Option<Installation> {
        &self.installation
    }

    /// Returns the check run event's sender.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn sender(&self) -> &Account {
        &self.sender
    }
}

impl Display for CheckRunAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string_representation = match self {
            CheckRunAction::Created => "created",
            CheckRunAction::Completed => "completed",
            CheckRunAction::Rerequested => "rerequested",
            CheckRunAction::RequestedAction => "requested action",
        };

        write!(f, "{}", string_representation)
    }
}

impl Display for CheckRunEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.check_run.name(), self.action)
    }
}

#[cfg(test)]
mod tests {
    use super::{CheckRunAction, CheckRunEvent};

    #[test]
    #[cfg(feature = "serde")]
    fn trait_deserialize() {
        let check_run_event: CheckRunEvent = serde_json::from_str(include_str!(
            "../../tests/fixtures/event/check_run.completed.json"
        ))
        .unwrap();

        assert!(matches!(
            check_run_event.action(),
            CheckRunAction::Completed
        ));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn trait_display() {
        let check_run_event: CheckRunEvent = serde_json::from_str(include_str!(
            "../../tests/fixtures/event/check_run.completed.json"
        ))
        .unwrap();

        assert_eq!("Run tests (completed)", check_run_event.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<CheckRunEvent>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<CheckRunEvent>();
    }
}
