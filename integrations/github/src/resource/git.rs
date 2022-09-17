use crate::name;

name!(
    /// Git reference
    ///
    /// A Git reference (git ref) is a file that contains a Git commit SHA-1 hash. When referring to
    /// a Git commit, you can use the Git reference, which is an easy-to-remember name, rather than
    /// the hash.
    ///
    /// Read more: https://docs.github.com/en/rest/git/refs
    GitRef
);

name!(
    /// Git commit SHA-1
    ///
    /// Commits in Git are uniquely identified by their SHA-1 hash, which is used throughout
    /// GitHub's API to reference commits in the Git database.
    GitSha
);
