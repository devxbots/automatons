use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use url::Url;

use crate::name;
use crate::resource::NodeId;

name!(
    /// License key
    ///
    /// Licenses have a unique key that identifies them.
    LicenseKey
);

name!(
    /// License name
    ///
    /// Licenses have a human-readable name.
    LicenseName
);

name!(
    /// SPDX identifier
    ///
    /// The Software Package Data Exchange (SDPX) maintains a list of licenses and assigns a unique
    /// identifier to each license. This identifier is used by many tools and platforms to identify
    /// licenses and exchange license information.
    SpdxId
);

/// Software license
///
/// GitHub tries to detect the license of a project automatically. It checks for a license file and
/// matches that against a known list of licenses, or reads the license fields in the package's
/// manifest, e.g. in `package.json` or `Cargo.toml`.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct License {
    key: LicenseKey,
    name: LicenseName,
    spdx_id: SpdxId,
    url: Url,
    node_id: NodeId,
}

impl License {
    /// Returns the license's key.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn key(&self) -> &LicenseKey {
        &self.key
    }

    /// Returns the license's name.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn name(&self) -> &LicenseName {
        &self.name
    }

    /// Returns the license's SPDX identifier.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn spdx_id(&self) -> &SpdxId {
        &self.spdx_id
    }

    /// Returns the API endpoint to query the license.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Returns the license's node id.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn node_id(&self) -> &NodeId {
        &self.node_id
    }
}

impl Display for License {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use url::Url;

    use crate::resource::NodeId;

    use super::{License, LicenseKey, LicenseName, SpdxId};

    #[test]
    fn trait_deserialize() {
        let license: License =
            serde_json::from_str(include_str!("../../tests/fixtures/resource/license.json"))
                .unwrap();

        assert_eq!("apache-2.0", license.key().get());
    }

    #[test]
    fn trait_display() {
        let license = License {
            key: LicenseKey::new("apache-2.0"),
            name: LicenseName::new("Apache License 2.0"),
            spdx_id: SpdxId::new("Apache-2.0"),
            url: Url::parse("https://api.github.com/licenses/apache-2.0").unwrap(),
            node_id: NodeId::new("MDc6TGljZW5zZTI="),
        };

        assert_eq!("Apache License 2.0", license.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<License>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<License>();
    }
}
