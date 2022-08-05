use std::fmt::{Display, Formatter};

use crate::resource::{GitRef, GitSha, MinimalRepository};

/// Pull request branch reference
///
/// Pull requests have a `base` branch against which they are opened, and a `head` branch that
/// contains the changes that should be merged. The [`PullRequest`] resource references these using
/// the [`PullRequestBranch`] data type.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct PullRequestBranch {
    #[cfg_attr(feature = "serde", serde(rename = "ref"))]
    git_ref: GitRef,

    #[cfg_attr(feature = "serde", serde(rename = "sha"))]
    git_sha: GitSha,

    repo: MinimalRepository,
}

impl PullRequestBranch {
    /// Returns the pull request branch's git ref.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn git_ref(&self) -> &GitRef {
        &self.git_ref
    }

    /// Returns the pull request branch's git sha.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn git_sha(&self) -> &GitSha {
        &self.git_sha
    }

    /// Returns the repository in which the pull request was created.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn repository(&self) -> &MinimalRepository {
        &self.repo
    }
}

impl Display for PullRequestBranch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.git_ref)
    }
}

#[cfg(test)]
mod test {
    use super::PullRequestBranch;

    const JSON: &str = r#"
    {
        "ref": "main",
        "sha": "3de05046636de664eff97823e24c92d382fa6607",
        "repo": {
            "id": 518377950,
            "url": "https://api.github.com/repos/devxbots/automatons",
            "name": "automatons"
        }
    }
    "#;

    #[test]
    #[cfg(feature = "serde")]
    fn trait_deserialize() {
        let branch: PullRequestBranch = serde_json::from_str(JSON).unwrap();

        assert_eq!("main", branch.git_ref().get());
    }

    #[test]
    fn trait_display() {
        let branch: PullRequestBranch = serde_json::from_str(JSON).unwrap();

        assert_eq!("main", branch.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<PullRequestBranch>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<PullRequestBranch>();
    }
}
