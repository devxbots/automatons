use std::fmt::{Display, Formatter};

/// Check run conclusion
///
/// When a check run finishes, its conclusion indicates the success or failure of the check run to
/// the user. Branch protection rules can be created to require a successful conclusion before code
/// can be merged into a branch.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum CheckRunConclusion {
    /// Check run finished successfully
    Success,

    /// Check run failed
    Failure,

    /// Check run finished in a neutral state
    Neutral,

    /// Check run was skipped
    Skipped,

    /// Check run was cancelled
    Cancelled,

    /// Check run timed out
    TimedOut,

    /// Check run requested an action from the user
    ActionRequired,

    /// Check run was marked as stable by GitHub
    Stale,
}

impl Display for CheckRunConclusion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string_representation = match self {
            CheckRunConclusion::Success => "success",
            CheckRunConclusion::Failure => "failure",
            CheckRunConclusion::Neutral => "neutral",
            CheckRunConclusion::Skipped => "skipped",
            CheckRunConclusion::Cancelled => "cancelled",
            CheckRunConclusion::TimedOut => "timed out",
            CheckRunConclusion::ActionRequired => "action required",
            CheckRunConclusion::Stale => "stale",
        };

        write!(f, "{}", string_representation)
    }
}

#[cfg(test)]
mod tests {
    use super::CheckRunConclusion;

    #[test]
    #[cfg(feature = "serde")]
    fn trait_deserialize() {
        let conclusion: CheckRunConclusion = serde_json::from_str(r#""action_required""#).unwrap();

        assert!(matches!(conclusion, CheckRunConclusion::ActionRequired));
    }

    #[test]
    fn trait_display() {
        let conclusion = CheckRunConclusion::ActionRequired;

        assert_eq!("action required", conclusion.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<CheckRunConclusion>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<CheckRunConclusion>();
    }
}
