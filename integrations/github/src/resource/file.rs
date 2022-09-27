use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use url::Url;

use crate::resource::GitSha;

/// File in a repository
///
/// Git repositories store files and directories. They can be queried using GitHub's [contents] API.
/// The API returns a file object with a set of metadata, e.g. the file size, name, and path. The
/// file's content is embedded in the response up to a certain size, and encoded using the file's
/// encoding.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct File {
    name: String,
    path: String,
    #[serde(with = "serde_bytes")]
    content: Vec<u8>,
    sha: GitSha,
    url: Url,
    git_url: Url,
    html_url: Url,
    download_url: Url,
}

impl File {
    /// Initializes a new file
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        name: String,
        path: String,
        content: Vec<u8>,
        sha: GitSha,
        url: Url,
        git_url: Url,
        html_url: Url,
        download_url: Url,
    ) -> Self {
        Self {
            name,
            path,
            content,
            sha,
            url,
            git_url,
            html_url,
            download_url,
        }
    }

    /// Returns the file name.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns the path of the file.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn path(&self) -> &String {
        &self.path
    }

    /// Returns the file contents.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn content(&self) -> &[u8] {
        &self.content
    }

    /// Returns the SHA of the Git commit to which the file belongs.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn sha(&self) -> &GitSha {
        &self.sha
    }

    /// Returns the API endpoint to query the file.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Returns the API endpoint to query the file's Git commit.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn git_url(&self) -> &Url {
        &self.git_url
    }

    /// Returns the URL to the account.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn html_url(&self) -> &Url {
        &self.html_url
    }

    /// Returns a temporary URL to download the file.
    ///
    /// Download URLs expire and are meant to be used just once. To ensure the download URL does not
    /// expire, please use the contents API to obtain a fresh download URL for each download.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn download_url(&self) -> &Url {
        &self.download_url
    }
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}

#[cfg(test)]
mod tests {
    use url::Url;

    use super::File;

    #[rustfmt::skip]
    fn file() -> File {
        File {
            name: "README.md".into(),
            path: "README.md".into(),
            content: "ZW5jb2RlZCBjb250ZW50IC4uLg==".into(),
            sha: "3d21ec53a331a6f037a91c368710b99387d012c1".into(),
            url: Url::parse("https://api.github.com/repos/octokit/octokit.rb/contents/README.md").unwrap(),
            git_url: Url::parse("https://api.github.com/repos/octokit/octokit.rb/git/blobs/3d21ec53a331a6f037a91c368710b99387d012c1").unwrap(),
            html_url: Url::parse("https://github.com/octokit/octokit.rb/blob/master/README.md").unwrap(),
            download_url: Url::parse("https://raw.githubusercontent.com/octokit/octokit.rb/master/README.md").unwrap(),
        }
    }

    #[test]
    fn trait_deserialize() {
        let json = r#"
        {
          "type": "file",
          "encoding": "base64",
          "size": 5362,
          "name": "README.md",
          "path": "README.md",
          "content": "ZW5jb2RlZCBjb250ZW50IC4uLg==",
          "sha": "3d21ec53a331a6f037a91c368710b99387d012c1",
          "url": "https://api.github.com/repos/octokit/octokit.rb/contents/README.md",
          "git_url": "https://api.github.com/repos/octokit/octokit.rb/git/blobs/3d21ec53a331a6f037a91c368710b99387d012c1",
          "html_url": "https://github.com/octokit/octokit.rb/blob/master/README.md",
          "download_url": "https://raw.githubusercontent.com/octokit/octokit.rb/master/README.md",
          "_links": {
            "git": "https://api.github.com/repos/octokit/octokit.rb/git/blobs/3d21ec53a331a6f037a91c368710b99387d012c1",
            "self": "https://api.github.com/repos/octokit/octokit.rb/contents/README.md",
            "html": "https://github.com/octokit/octokit.rb/blob/master/README.md"
          }
        }
        "#;

        let file: File = serde_json::from_str(json).unwrap();

        assert_eq!("README.md", file.name());
    }

    #[test]
    fn trait_display() {
        assert_eq!("README.md", file().to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<File>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<File>();
    }
}
