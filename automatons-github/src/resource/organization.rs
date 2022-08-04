use std::fmt::{Display, Formatter};

use url::Url;

use crate::id;
use crate::resource::{Login, NodeId};

id!(
    /// Organization id
    ///
    /// The [`OrganizationId`] is a unique, numerical id that is used to interact with an
    /// organization through [GitHub's REST API](https://docs.github.com/en/rest).
    OrganizationId
);

/// Organization
///
/// Organizations enable users to collaborate and share resources with each other in a structured
/// way. Organizations can have members, teams, repositories, and other resources.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Organization {
    login: Login,
    id: OrganizationId,
    node_id: NodeId,
    url: Url,
    repos_url: Url,
    events_url: Url,
    hooks_url: Url,
    issues_url: Url,
    members_url: Url,
    public_members_url: Url,
    avatar_url: Url,
    description: String,
}

impl Organization {
    /// Returns the organization's [`Login`].
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn login(&self) -> &Login {
        &self.login
    }

    /// Returns the organization's [`OrganizationId`].
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn id(&self) -> OrganizationId {
        self.id
    }

    /// Returns the organization's [`NodeId`].
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn node_id(&self) -> &NodeId {
        &self.node_id
    }

    /// Returns the API endpoint to query the organization.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Returns the API endpoint to query the organization's repositories.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn repos_url(&self) -> &Url {
        &self.repos_url
    }

    /// Returns the API endpoint to query the organization's events.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn events_url(&self) -> &Url {
        &self.events_url
    }

    /// Returns the API endpoint to query the organization's hooks.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn hooks_url(&self) -> &Url {
        &self.hooks_url
    }

    /// Returns the API endpoint to query the organization's issues.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn issues_url(&self) -> &Url {
        &self.issues_url
    }

    /// Returns the API endpoint to query the organization's members.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn members_url(&self) -> &Url {
        &self.members_url
    }

    /// Returns the API endpoint to query the organization's public members.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn public_members_url(&self) -> &Url {
        &self.public_members_url
    }

    /// Returns the URL to the organization's avatar.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn avatar_url(&self) -> &Url {
        &self.avatar_url
    }

    /// Returns the organization's description.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn description(&self) -> &str {
        &self.description
    }
}

impl Display for Organization {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.login)
    }
}

#[cfg(test)]
mod tests {
    use super::Organization;

    #[test]
    #[cfg(feature = "serde")]
    fn trait_deserialize() {
        let organization: Organization = serde_json::from_str(include_str!(
            "../../tests/fixtures/resource/organization.json"
        ))
        .unwrap();

        assert_eq!("devxbots", organization.login().get());
    }

    #[test]
    #[cfg(feature = "serde")]
    fn trait_display() {
        let organization: Organization = serde_json::from_str(include_str!(
            "../../tests/fixtures/resource/organization.json"
        ))
        .unwrap();

        assert_eq!("devxbots", organization.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Organization>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Organization>();
    }
}
