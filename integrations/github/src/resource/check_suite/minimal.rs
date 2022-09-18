use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::resource::CheckSuiteId;

/// Minimal representation of a [`CheckSuite`]
///
/// GitHub truncates data types in some API responses and webhook events to reduce the payload size.
/// The [`MinimalCheckSuite`] represents a `[CheckSuite`], but contains only the most basic fields.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct MinimalCheckSuite {
    id: CheckSuiteId,
}

impl MinimalCheckSuite {
    /// Returns the check suite's id.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn id(&self) -> CheckSuiteId {
        self.id
    }
}

impl Display for MinimalCheckSuite {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::{CheckSuiteId, MinimalCheckSuite};

    const JSON: &str = r#"
    {
        "id": 5
    }
    "#;

    #[test]
    fn trait_deserialize() {
        let check_suite: MinimalCheckSuite = serde_json::from_str(JSON).unwrap();

        assert_eq!(5, check_suite.id().get());
    }

    #[test]
    fn trait_display() {
        let minimal_check_suite = MinimalCheckSuite {
            id: CheckSuiteId::new(5),
        };

        assert_eq!("5", minimal_check_suite.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<MinimalCheckSuite>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<MinimalCheckSuite>();
    }
}
