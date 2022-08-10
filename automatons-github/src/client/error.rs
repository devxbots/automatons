use thiserror::Error;

/// Errors that the GitHub Client can return
///
/// The GitHub Client provides authentication to tasks in this crate. Should requests fail due to
/// authentication issues, the client will return a typed error that the task can handle gracefully.
#[derive(Debug, Error)]
pub enum ClientError {
    #[error("{1}")]
    Configuration(#[source] Box<dyn std::error::Error + Send + Sync>, String),

    #[allow(dead_code)] // TODO: Investigate why this variant is reported as unused
    #[error("failed to find the requested resource")]
    NotFound,

    #[error(transparent)]
    Request(#[from] reqwest::Error),

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
