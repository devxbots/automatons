//! Events on GitHub
//!
//! The GitHub integration enables the [automatons] framework to interact with webhook sevents on
//! GitHub. These events are modelled as strongly-typed data types in this module, and each model
//! maps to a [webhook event](https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads)
//! with its payload.
//!
//! [automatons]: https://github.com/devxbots/automatons

use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

pub use self::check_run::{CheckRunAction, CheckRunEvent};

mod check_run;

/// Event on GitHub
///
/// Webhooks allow you to build or set up integrations, such as GitHub Apps or OAuth Apps, which
/// subscribe to certain events on GitHub.com. When one of those events is triggered, we'll send a
/// HTTP POST payload to the webhook's configured URL. Webhooks can be used to update an external
/// issue tracker, trigger CI builds, update a backup mirror, or even deploy to your production
/// server. You're only limited by your imagination.
///
/// Read more: https://docs.github.com/en/developers/webhooks-and-events/webhooks/about-webhooks
///
/// The webhook payloads are inside a [`Box`], since their sizes vary greatly.
#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum GitHubEvent {
    /// Check run event
    CheckRun(Box<CheckRunEvent>),

    /// Unsupported event
    Unsupported,
}

impl Display for GitHubEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string_representation = match self {
            GitHubEvent::CheckRun(event) => format!("check run {}", event.action()),
            GitHubEvent::Unsupported => "unsupported".into(),
        };

        write!(f, "{}", string_representation)
    }
}

impl Default for GitHubEvent {
    fn default() -> Self {
        GitHubEvent::Unsupported
    }
}

#[cfg(test)]
mod tests {
    use super::GitHubEvent;

    #[test]
    fn trait_deserialize_check_run() {
        let github_event: GitHubEvent = serde_json::from_str(include_str!(
            "../../tests/fixtures/event/check_run.completed.json"
        ))
        .unwrap();

        if let GitHubEvent::CheckRun(check_run_event) = github_event {
            assert_eq!("Run tests", check_run_event.check_run().name().get());
        } else {
            panic!("expected a check run event");
        }
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<GitHubEvent>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<GitHubEvent>();
    }
}
