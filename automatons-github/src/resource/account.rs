use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use url::Url;

use crate::resource::NodeId;
use crate::{id, name};

id!(
    /// Unique account id
    ///
    /// The [`AccountId`] is a unique, numerical id that is used to interact with an account through
    /// [GitHub's REST API](https://docs.github.com/en/rest).
    AccountId
);

name!(
    /// Unique account name
    ///
    /// Accounts on GitHub have a unique, human-readable name that is used throughout GitHub's
    /// website.
    Login
);

/// GitHub account type
///
/// GitHub differentiates between three different kinds of accounts: bots, organizations, and users.
/// Bots represent (third-party) integrations with the platform, often driven by GitHub Apps.
/// Organizations provide a space for users to collaborate and share resources. And user accounts
/// represent the humans that build software on GitHub.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub enum AccountType {
    /// Bot account
    Bot,

    /// Organization account
    Organization,

    /// User account
    User,
}

/// GitHub account
///
/// GitHub references accounts in many events and API responses. Accounts are a lightweight
/// representation of three other resources: bots, users, and organizations. They provide a unified
/// interface to information that is shared between all account types, and hide a lot of information
/// that would unnecessarily increase payload sizes.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct Account {
    login: Login,
    id: AccountId,
    node_id: NodeId,
    avatar_url: Url,
    url: Url,
    html_url: Url,
    followers_url: Url,
    following_url: Url,
    gists_url: Url,
    starred_url: Url,
    subscriptions_url: Url,
    organizations_url: Url,
    repos_url: Url,
    events_url: Url,
    received_events_url: Url,
    site_admin: bool,

    #[serde(rename = "type")]
    account_type: AccountType,
}

impl Account {
    /// Returns the account's unique [`Login`].
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn login(&self) -> &Login {
        &self.login
    }

    /// Returns the account's unique [`AccountId`].
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn id(&self) -> AccountId {
        self.id
    }

    /// Returns the account's unique [`NodeId`].
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn node_id(&self) -> &NodeId {
        &self.node_id
    }

    /// Returns the URl to the account's avatar.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn avatar_url(&self) -> &Url {
        &self.avatar_url
    }

    /// Returns the API endpoint to query the account.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Returns the URL to the account.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn html_url(&self) -> &Url {
        &self.html_url
    }

    /// Returns the API endpoint to query the account's followers.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn followers_url(&self) -> &Url {
        &self.followers_url
    }

    /// Returns the API endpoint to query the accounts that this account follows.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn following_url(&self) -> &Url {
        &self.following_url
    }

    /// Returns the API endpoint to query the account's gists.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn gists_url(&self) -> &Url {
        &self.gists_url
    }

    /// Returns the API endpoint to query the repositories that the account has starred.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn starred_url(&self) -> &Url {
        &self.starred_url
    }

    /// Returns the API endpoint to query the account's subscriptions.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn subscriptions_url(&self) -> &Url {
        &self.subscriptions_url
    }

    /// Returns the API endpoint to query the account's organizations.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn organizations_url(&self) -> &Url {
        &self.organizations_url
    }

    /// Returns the API endpoint to query the account's repositories.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn repos_url(&self) -> &Url {
        &self.repos_url
    }

    /// Returns the API endpoint to query the account's events.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn events_url(&self) -> &Url {
        &self.events_url
    }

    /// Returns the API endpoint to query the events that the account has received.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn received_events_url(&self) -> &Url {
        &self.received_events_url
    }

    /// Indicates whether the account is a site admin.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn site_admin(&self) -> bool {
        self.site_admin
    }
}

impl Display for Account {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.login)
    }
}

#[cfg(test)]
mod tests {
    use url::{ParseError, Url};

    use crate::resource::account::AccountType;

    use super::Account;

    #[rustfmt::skip]
    fn account() -> Result<Account, ParseError> {
        Ok(Account {
            login: "dependabot[bot]".into(),
            id: 49699333.into(),
            node_id: "MDM6Qm90NDk2OTkzMzM=".into(),
            avatar_url: Url::parse("https://avatars.githubusercontent.com/in/29110?v=4")?,
            url: Url::parse("https://api.github.com/users/dependabot%5Bbot%5D")?,
            html_url: Url::parse("https://github.com/apps/dependabot")?,
            followers_url: Url::parse("https://api.github.com/users/dependabot%5Bbot%5D/followers")?,
            following_url: Url::parse("https://api.github.com/users/dependabot%5Bbot%5D/following{/other_user}")?,
            gists_url: Url::parse("https://api.github.com/users/dependabot%5Bbot%5D/gists{/gist_id}")?,
            starred_url: Url::parse("https://api.github.com/users/dependabot%5Bbot%5D/starred{/owner}{/repo}")?,
            subscriptions_url: Url::parse("https://api.github.com/users/dependabot%5Bbot%5D/subscriptions")?,
            organizations_url: Url::parse("https://api.github.com/users/dependabot%5Bbot%5D/orgs")?,
            repos_url: Url::parse("https://api.github.com/users/dependabot%5Bbot%5D/repos")?,
            events_url: Url::parse("https://api.github.com/users/dependabot%5Bbot%5D/events{/privacy}")?,
            received_events_url: Url::parse("https://api.github.com/users/dependabot%5Bbot%5D/received_events")?,
            account_type: AccountType::Bot,
            site_admin: false,
        })
    }

    #[test]
    fn trait_deserialize() {
        let json = r#"
        {
            "login": "dependabot[bot]",
            "id": 49699333,
            "node_id": "MDM6Qm90NDk2OTkzMzM=",
            "avatar_url": "https://avatars.githubusercontent.com/in/29110?v=4",
            "gravatar_id": "",
            "url": "https://api.github.com/users/dependabot%5Bbot%5D",
            "html_url": "https://github.com/apps/dependabot",
            "followers_url": "https://api.github.com/users/dependabot%5Bbot%5D/followers",
            "following_url": "https://api.github.com/users/dependabot%5Bbot%5D/following{/other_user}",
            "gists_url": "https://api.github.com/users/dependabot%5Bbot%5D/gists{/gist_id}",
            "starred_url": "https://api.github.com/users/dependabot%5Bbot%5D/starred{/owner}{/repo}",
            "subscriptions_url": "https://api.github.com/users/dependabot%5Bbot%5D/subscriptions",
            "organizations_url": "https://api.github.com/users/dependabot%5Bbot%5D/orgs",
            "repos_url": "https://api.github.com/users/dependabot%5Bbot%5D/repos",
            "events_url": "https://api.github.com/users/dependabot%5Bbot%5D/events{/privacy}",
            "received_events_url": "https://api.github.com/users/dependabot%5Bbot%5D/received_events",
            "type": "Bot",
            "site_admin": false
        }
        "#;

        let account: Account = serde_json::from_str(json).unwrap();

        assert_eq!("dependabot[bot]", account.login().get());
    }

    #[test]
    fn trait_display() {
        let account: Account = account().unwrap();

        assert_eq!("dependabot[bot]", account.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Account>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Account>();
    }
}
