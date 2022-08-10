use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use url::Url;

use crate::resource::{RepositoryId, RepositoryName};

/// Minimal representation of a [`Repository`]
///
/// GitHub truncates data types in some API responses and webhook events to reduce the payload size.
/// The [`MinimalRepository`] represents a `[Repository`], but contains only the most basic fields.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct MinimalRepository {
    id: RepositoryId,
    name: RepositoryName,
    url: Url,
}

impl MinimalRepository {
    /// Returns the repository's id.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn id(&self) -> RepositoryId {
        self.id
    }

    /// Returns the repository's name.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn name(&self) -> &RepositoryName {
        &self.name
    }

    /// Returns the API endpoint to query the repository.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn url(&self) -> &Url {
        &self.url
    }
}

impl Display for MinimalRepository {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = self
            .url
            .path_segments()
            .and_then(|segments| {
                let user_and_repo: Vec<&str> = segments.rev().take(2).collect();

                if user_and_repo.len() != 2 {
                    return None;
                }

                Some(
                    user_and_repo
                        .into_iter()
                        .rev()
                        .collect::<Vec<&str>>()
                        .join("/"),
                )
            })
            .unwrap_or_else(|| self.name.to_string());

        write!(f, "{}", name)
    }
}

#[cfg(test)]
mod tests {
    use url::Url;

    use super::{MinimalRepository, RepositoryId, RepositoryName};

    const JSON: &str = r#"
    {
        "id": 518377950,
        "url": "https://api.github.com/repos/devxbots/automatons",
        "name": "automatons"
    }
    "#;

    #[test]
    fn trait_deserialize() {
        let repository: MinimalRepository = serde_json::from_str(JSON).unwrap();

        assert_eq!("automatons", repository.name().get());
    }

    #[test]
    fn trait_display() {
        let repository = MinimalRepository {
            id: RepositoryId::new(518377950),
            name: RepositoryName::new("automatons"),
            url: Url::parse("https://api.github.com/repos/devxbots/automatons").unwrap(),
        };

        assert_eq!("devxbots/automatons", repository.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<MinimalRepository>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<MinimalRepository>();
    }
}
