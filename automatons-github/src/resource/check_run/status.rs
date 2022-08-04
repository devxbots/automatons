use std::fmt::{Display, Formatter};

/// Check run status
///
/// Check runs can be in one of three states. When a check run is created, the status is `queued`.
/// Once its dependencies are ready and the execution starts, the status changes to `in progress`.
/// Finally, the check run is finished and the status is set to `completed`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum CheckRunStatus {
    /// Queued state
    Queued,

    /// In progress state
    InProgress,

    /// Completed state
    Completed,
}

impl Display for CheckRunStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string_representation = match self {
            CheckRunStatus::Queued => "queued",
            CheckRunStatus::InProgress => "in progress",
            CheckRunStatus::Completed => "completed",
        };

        write!(f, "{}", string_representation)
    }
}

#[cfg(test)]
mod tests {
    use super::CheckRunStatus;

    #[test]
    #[cfg(feature = "serde")]
    fn trait_deserialize() {
        let status: CheckRunStatus = serde_json::from_str(r#""in_progress""#).unwrap();

        assert!(matches!(status, CheckRunStatus::InProgress));
    }

    #[test]
    fn trait_display() {
        let status = CheckRunStatus::InProgress;

        assert_eq!("in progress", status.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<CheckRunStatus>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<CheckRunStatus>();
    }
}
