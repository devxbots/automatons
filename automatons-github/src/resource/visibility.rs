use std::fmt::{Display, Formatter};

/// Visibility of a resource
///
/// Some resources on GitHub can be made available to different groups of people. Most prominently,
/// the visibility of repositories determines who can access a repository and its nested resources.
/// On [github.com](https://github.com), resources can either be `public` or `private`. On hosted
/// GitHub Enterprise servers, `internal` resources can only be access by members of the same
/// GitHub organization.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Visibility {
    /// Internal visibility
    ///
    /// This resource is visible only to people in the same organization.
    Internal,

    /// Private visibility
    ///
    /// This resource is hidden from the public.
    Private,

    /// Public visibility
    ///
    /// This resource is public.
    Public,
}

impl Display for Visibility {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string_representation = match self {
            Visibility::Internal => "internal",
            Visibility::Private => "private",
            Visibility::Public => "public",
        };

        write!(f, "{}", string_representation)
    }
}

#[cfg(test)]
mod tests {
    use super::Visibility;

    #[test]
    #[cfg(feature = "serde")]
    fn trait_deserialize() {
        let visibility: Visibility = serde_json::from_str(r#""internal""#).unwrap();

        assert!(matches!(visibility, Visibility::Internal));
    }

    #[test]
    fn trait_display() {
        let visibility = Visibility::Private;

        assert_eq!("private", visibility.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Visibility>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Visibility>();
    }
}
