use thiserror::Error;

/// Errors that an automaton can return
///
/// When an automaton executes its tasks, things can fail. Automatons might interact with external
/// resources or third-party APIs that are unreliable, the configuration might be wrong, or
/// something totally unexpected might happen. In these situations, automatons can abort their
/// execution and return an [`Error`].
///
/// # Example
///
/// The implementation of the error type is based on [thiserror] and [anyhow]. Unexpected errors can
/// easily be converted to an error by calling `.context` on the original error.
///
/// ```rust
/// use std::io::ErrorKind;
///
/// use anyhow::Context;
/// use automatons::Error;
///
/// fn connect() -> Result<(), Error> {
///     let failure = Err(std::io::Error::new(ErrorKind::TimedOut, "connection timed out"));
///     let error = failure.context("failed to connect to API due to connection time out")?;
///
///     Ok(())
/// }
/// ```
///
/// anyhow: https://crates.io/crates/anyhow
/// thiserror: https://crates.io/crates/thiserror
#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Configuration(String),

    #[cfg(feature = "sqlx")]
    #[error("{0}")]
    Database(#[from] sqlx::Error),

    #[error("failed to find resource at {0}")]
    NotFound(String),

    #[cfg(feature = "reqwest")]
    #[error(transparent)]
    Request(#[from] reqwest::Error),

    #[error("{0}")]
    Serialization(String),

    #[error("{0}")]
    UnsupportedEvent(String),

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
