/// Generate an identifier type
///
/// GitHub uses unique numerical ids for most of its resources. The [`id!`] macro generates a struct
/// that represents such an identifier, making it easy to generate unique types for different
/// resources.
///
/// # Example
///
/// ```rust
/// use automatons_github::id;
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
        #[derive(serde::Deserialize, serde::Serialize)]
        pub struct $id(u64);

        #[allow(dead_code)]
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
/// use automatons_github::name;
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
        #[derive(serde::Deserialize, serde::Serialize)]
        pub struct $name(String);

        #[allow(dead_code)]
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

/// Generate a secret type
///
/// GitHub Apps have several secrets that must be configured, for example the app's private key and
/// webhook secret. The [`secret`] macro can generate a type for these secrets that both protects
/// the value from accidental exposure and allows the Rust compiler to enforce its type safety.
///
/// # Example
///
/// ```rust
/// use automatons_github::secret;
///
/// secret!(PrivateKey);
/// secret!(WebhookSecret);
/// ```
#[macro_export]
macro_rules! secret {
    (
        $(#[$meta:meta])*
        $secret:ident
    ) => {
        $(#[$meta])*
        #[derive(Clone, Debug)]
        pub struct $secret(secrecy::SecretString);

        #[allow(dead_code)]
        impl $secret {
            /// Initializes a new secret.
            #[cfg_attr(feature = "tracing", tracing::instrument)]
            pub fn new(secret: &str) -> Self {
                Self(secrecy::SecretString::new(String::from(secret)))
            }

            /// Returns the inner value of the secret.
            #[cfg_attr(feature = "tracing", tracing::instrument)]
            pub fn expose(&self) -> &str {
                use secrecy::ExposeSecret;
                self.0.expose_secret()
            }
        }

        impl std::fmt::Display for $secret {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "[REDACTED]")
            }
        }

        impl From<&str> for $secret {
            #[cfg_attr(feature = "tracing", tracing::instrument)]
            fn from(secret: &str) -> $secret {
                $secret(secrecy::SecretString::new(String::from(secret)))
            }
        }

        impl From<String> for $secret {
            #[cfg_attr(feature = "tracing", tracing::instrument)]
            fn from(secret: String) -> $secret {
                $secret(secrecy::SecretString::new(secret))
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

    secret!(
        /// Secret for tests
        TestSecret
    );

    #[test]
    fn secret() {
        let secret = TestSecret::new("test");

        assert_eq!("test", secret.expose());
        assert_eq!("[REDACTED]", secret.to_string());
    }

    #[test]
    fn secret_from_str() {
        let _secret: TestSecret = "test".into();
    }

    #[test]
    fn secret_from_string() {
        let _secret: TestSecret = String::from("test").into();
    }
}
