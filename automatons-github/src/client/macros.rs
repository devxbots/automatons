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

        impl $secret {
            /// Initializes a new secret.
            #[cfg_attr(feature = "tracing", tracing::instrument)]
            pub fn new(secret: &str) -> Self {
                Self(secrecy::SecretString::new(String::from(secret)))
            }

            /// Returns the inner value of the secret.
            #[cfg_attr(feature = "tracing", tracing::instrument)]
            pub fn get(&self) -> &str {
                use secrecy::ExposeSecret;
                self.0.expose_secret()
            }
        }

        impl std::fmt::Display for $secret {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self.0)
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
