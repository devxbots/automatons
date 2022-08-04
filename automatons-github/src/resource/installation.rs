use crate::id;
use crate::resource::NodeId;
use std::fmt::{Display, Formatter};

id!(
    /// Installation id
    ///
    /// The [`InstallationId`] is a unique, numerical id that is used to interact with an
    /// installation through [GitHub's REST API](https://docs.github.com/en/rest).
    InstallationId
);

/// App installation
///
/// When a user adds a GitHub App to an account, a new app installation is created. The installation
/// id can be used by the app to request a scoped access token that allows it to interact with the
/// resources of the account.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Installation {
    id: InstallationId,
    node_id: NodeId,
}

impl Installation {
    /// Returns the installation's id.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn id(&self) -> InstallationId {
        self.id
    }

    /// Returns the installation's node id.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn node_id(&self) -> &NodeId {
        &self.node_id
    }
}

impl Display for Installation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::{Installation, InstallationId};
    use crate::resource::NodeId;

    #[test]
    #[cfg(feature = "serde")]
    fn trait_deserialize() {
        let installation: Installation = serde_json::from_str(include_str!(
            "../../tests/fixtures/resource/installation.json"
        ))
        .unwrap();

        assert_eq!(25802826, installation.id().get());
    }

    #[test]
    fn trait_display() {
        let installation = Installation {
            id: InstallationId::new(42),
            node_id: NodeId::new("node_id"),
        };

        assert_eq!("42", installation.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Installation>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Installation>();
    }
}
