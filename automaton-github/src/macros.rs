/// Generate an identifier type
///
/// GitHub uses unique numerical ids for most of its resources. The [`id!`] macro generates a struct
/// that represents such an identifier, making it easy to generate unique types for different
/// resources.
///
/// # Example
///
/// ```rust
/// use automaton_github::id;
///
/// id!(RepositoryId);
/// id!(UserId);
/// ```
#[macro_export]
macro_rules! id {
    (
        $(#[$meta:meta])*
        $id:ident
    ) => {
        $(#[$meta])*
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        pub struct $id(u64);

        impl $id {
            /// Initializes a new id.
            #[cfg_attr(feature = "tracing", tracing::instrument)]
            pub fn new(id: u64) -> Self {
                Self(id)
            }

            /// Returns the inner value of the id.
            #[cfg_attr(feature = "tracing", tracing::instrument)]
            pub fn get(&self) -> u64 {
                self.0
            }
        }

        impl std::fmt::Display for $id {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<u64> for $id {
            #[cfg_attr(feature = "tracing", tracing::instrument)]
            fn from(id: u64) -> $id {
                $id(id)
            }
        }
    };
}

/// Generate a resource name
///
/// Many resources on GitHub have unique names that identify them. For example, user names are
/// unique across the platform. Since these names can be used as identifiers, it is recommended to
/// encode them using the Rust type system. This avoids passing them around as strings, and
/// eventually using them in the wrong place.
///
/// The [`name!`] macro makes it easy to generate a newtype that represents a specific name.
///
/// # Example
///
/// ```rust
/// use automaton_github::name;
///
/// name!(RepositoryName);
/// name!(UserName);
/// ```
#[macro_export]
macro_rules! name {
    (
        $(#[$meta:meta])*
        $name:ident
    ) => {
        $(#[$meta])*
        #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        pub struct $name(String);

        impl $name {
            /// Initializes a new name.
            #[cfg_attr(feature = "tracing", tracing::instrument)]
            pub fn new(name: &str) -> Self {
                Self(name.into())
            }

            /// Returns the inner value of the name.
            #[cfg_attr(feature = "tracing", tracing::instrument)]
            pub fn get(&self) -> &str {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<&str> for $name {
            #[cfg_attr(feature = "tracing", tracing::instrument)]
            fn from(string: &str) -> $name {
                $name(string.into())
            }
        }

        impl From<String> for $name {
            #[cfg_attr(feature = "tracing", tracing::instrument)]
            fn from(string: String) -> $name {
                $name(string)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{id, name};

    id!(
        /// Identifier for tests
        TestId
    );

    #[test]
    fn id() {
        let id = TestId::new(42);

        assert_eq!(42, id.get());
        assert_eq!("42", id.to_string());
    }

    #[test]
    fn id_from_u64() {
        let _id: TestId = 42.into();
    }

    name!(
        /// Name for tests
        TestName
    );

    #[test]
    fn name() {
        let name = TestName::new("test");

        assert_eq!("test", name.get());
        assert_eq!("test", name.to_string());
    }

    #[test]
    fn name_from_str() {
        let _name: TestName = "test".into();
    }

    #[test]
    fn name_from_string() {
        let _name: TestName = String::from("test").into();
    }
}
