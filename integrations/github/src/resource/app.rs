use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::resource::{Account, NodeId};
use crate::{id, name};

id!(
    /// App id
    ///
    /// The [`AppId`] is a unique, numerical id that is used to interact with an app through
    /// [GitHub's REST API](https://docs.github.com/en/rest).
    AppId
);

name!(
    /// App name
    ///
    /// Apps on GitHub have a human-readable name that is used throughout GitHub's website as well
    /// as for status checks.
    AppName
);

name!(
    /// App slug
    ///
    /// The [`AppSlug`] is a URL-friendly version of the app's name.
    AppSlug
);

/// GitHub App
///
/// Third-parties can create integrations with the GitHub platform by creating a GitHub App. Apps
/// have their own identify and authentication, and can request granular permissions and access to
/// events. Every [`App`] is owned by an [`Account`].
#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct App {
    id: AppId,
    node_id: NodeId,
    name: AppName,
    slug: AppSlug,
    owner: Account,
    description: String,
    external_url: Url,
    html_url: Url,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    permissions: HashMap<String, String>,
    events: Vec<String>,
}

impl App {
    /// Returns the app's id.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn id(&self) -> AppId {
        self.id
    }

    /// Returns the app's node id.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn node_id(&self) -> &NodeId {
        &self.node_id
    }

    /// Returns the app's name.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn name(&self) -> &AppName {
        &self.name
    }

    /// Returns the app's slug.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn slug(&self) -> &AppSlug {
        &self.slug
    }

    /// Returns the app's owner.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn owner(&self) -> &Account {
        &self.owner
    }

    /// Returns the app's description.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn description(&self) -> &String {
        &self.description
    }

    /// Returns the URL to the app's external website.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn external_url(&self) -> &Url {
        &self.external_url
    }

    /// Returns the URL to the app's website on GitHub.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn html_url(&self) -> &Url {
        &self.html_url
    }

    /// Returns the date when the app was created.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    /// Returns the date when the app was last updated.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    /// Returns the app's permissions.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn permissions(&self) -> &HashMap<String, String> {
        &self.permissions
    }

    /// Returns the events to which the app is subscribed.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn events(&self) -> &Vec<String> {
        &self.events
    }
}

impl Display for App {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::App;

    #[test]
    fn trait_deserialize() {
        let app: App =
            serde_json::from_str(include_str!("../../tests/fixtures/resource/app.json")).unwrap();

        assert_eq!("devxbots/checkbot", app.name().get());
    }

    #[test]
    fn trait_display() {
        let app: App =
            serde_json::from_str(include_str!("../../tests/fixtures/resource/app.json")).unwrap();

        assert_eq!("devxbots/checkbot", app.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<App>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<App>();
    }
}
