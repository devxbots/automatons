use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::resource::{Account, License, NodeId, Visibility};
use crate::{id, name};

pub use self::minimal::MinimalRepository;

mod minimal;

id!(
    /// Repository id
    ///
    /// The [`RepositoryId`] is a unique, numerical id that is used to interact with an account
    /// through [GitHub's REST API](https://docs.github.com/en/rest).
    RepositoryId
);

name!(
    /// Repository name
    ///
    /// Repositories on GitHub have a human-readable name that is used throughout GitHub's
    /// website. The name is unique within the scope of its owner.
    RepositoryName
);

name!(
    /// Repository owner and name
    ///
    /// The full name of a repository is a unique combination of the repository's owner and name.
    RepositoryFullName
);

/// Repository on GitHub
///
/// Repositories are a core resource on GitHub, and most other resources belong to them. They are
/// uniquely identified by the combination of their `owner` and `name`.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct Repository {
    #[serde(flatten)]
    minimal: MinimalRepository,

    node_id: NodeId,
    owner: Account,
    full_name: RepositoryFullName,
    description: String,
    homepage: String,
    language: String,
    license: Option<License>,
    visibility: Visibility,
    default_branch: String,
    topics: Vec<String>,
    size: u64,
    stargazers_count: u64,
    watchers_count: u64,
    forks_count: u64,
    open_issues_count: u64,
    private: bool,
    fork: bool,
    has_issues: bool,
    has_projects: bool,
    has_wiki: bool,
    has_pages: bool,
    archived: bool,
    disabled: bool,
    allow_forking: bool,
    is_template: bool,
    web_commit_signoff_required: bool,
    html_url: Url,
    keys_url: Url,
    collaborators_url: Url,
    teams_url: Url,
    hooks_url: Url,
    issue_events_url: Url,
    events_url: Url,
    assignees_url: Url,
    branches_url: Url,
    tags_url: Url,
    blobs_url: Url,
    git_tags_url: Url,
    git_refs_url: Url,
    trees_url: Url,
    statuses_url: Url,
    languages_url: Url,
    stargazers_url: Url,
    contributors_url: Url,
    subscribers_url: Url,
    subscription_url: Url,
    commits_url: Url,
    git_commits_url: Url,
    comments_url: Url,
    issue_comment_url: Url,
    contents_url: Url,
    compare_url: Url,
    merges_url: Url,
    archive_url: Url,
    downloads_url: Url,
    issues_url: Url,
    pulls_url: Url,
    milestones_url: Url,
    notifications_url: Url,
    labels_url: Url,
    releases_url: Url,
    deployments_url: Url,
    git_url: Url,
    ssh_url: String,
    clone_url: Url,
    svn_url: Url,
    mirror_url: Option<Url>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    pushed_at: DateTime<Utc>,
}

impl Repository {
    /// Returns the repository's unique id.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn id(&self) -> RepositoryId {
        self.minimal.id()
    }

    /// Returns the repository's node id.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn node_id(&self) -> &NodeId {
        &self.node_id
    }

    /// Returns the repository's name.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn name(&self) -> &RepositoryName {
        self.minimal.name()
    }

    /// Returns the account which ows the repository.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn owner(&self) -> &Account {
        &self.owner
    }

    /// Returns the repository's full name.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn full_name(&self) -> &RepositoryFullName {
        &self.full_name
    }

    /// Returns the repository's description.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn description(&self) -> &String {
        &self.description
    }

    /// Returns the URL to the repository's homepage.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn homepage(&self) -> &String {
        &self.homepage
    }

    /// Returns the repository's primary programming language.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn language(&self) -> &String {
        &self.language
    }

    /// Returns the repository's license.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn license(&self) -> &Option<License> {
        &self.license
    }

    /// Returns the repository's visibility.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn visibility(&self) -> Visibility {
        self.visibility
    }

    /// Returns the repository's default branch.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn default_branch(&self) -> &String {
        &self.default_branch
    }

    /// Returns the repository's topics.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn topics(&self) -> &Vec<String> {
        &self.topics
    }

    /// Returns the repository's size.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Returns the repository's stargazers count.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn stargazers_count(&self) -> u64 {
        self.stargazers_count
    }

    /// Returns the repository's watchers count.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn watchers_count(&self) -> u64 {
        self.watchers_count
    }

    /// Returns the repository's forks count.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn forks_count(&self) -> u64 {
        self.forks_count
    }

    /// Returns the repository's open issues count.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn open_issues_count(&self) -> u64 {
        self.open_issues_count
    }

    /// Indicates whether the repository is private.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn private(&self) -> bool {
        self.private
    }

    /// Indicates whether the repository is a fork.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn fork(&self) -> bool {
        self.fork
    }

    /// Indicates whether the issues feature is enabled for the repository.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn has_issues(&self) -> bool {
        self.has_issues
    }

    /// Indicates whether the projects feature is enabled for the repository.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn has_projects(&self) -> bool {
        self.has_projects
    }

    /// Indicates whether the wiki feature is enabled for the repository.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn has_wiki(&self) -> bool {
        self.has_wiki
    }

    /// Indicates whether the repository has a static website.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn has_pages(&self) -> bool {
        self.has_pages
    }

    /// Indicates whether the repository has been archived.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn archived(&self) -> bool {
        self.archived
    }

    /// Indicates whether the repository has been disabled.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn disabled(&self) -> bool {
        self.disabled
    }

    /// Indicates whether the repository can be forked.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn allow_forking(&self) -> bool {
        self.allow_forking
    }

    /// Indicates whether the repository can be used as a template.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn is_template(&self) -> bool {
        self.is_template
    }

    /// Indicates whether the signoff is required for commits through GitHub's web interface.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn web_commit_signoff_required(&self) -> bool {
        self.web_commit_signoff_required
    }

    /// Returns the URL to the repository.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn html_url(&self) -> &Url {
        &self.html_url
    }

    /// Returns the API endpoint to query the repository.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn url(&self) -> &Url {
        self.minimal.url()
    }

    /// Returns the API endpoint to query the repository's keys.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn keys_url(&self) -> &Url {
        &self.keys_url
    }

    /// Returns the API endpoint to query the repository's collaborators.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn collaborators_url(&self) -> &Url {
        &self.collaborators_url
    }

    /// Returns the API endpoint to query the repository's teams.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn teams_url(&self) -> &Url {
        &self.teams_url
    }

    /// Returns the API endpoint to query the repository's hooks.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn hooks_url(&self) -> &Url {
        &self.hooks_url
    }

    /// Returns the API endpoint to query the repository's issue events.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn issue_events_url(&self) -> &Url {
        &self.issue_events_url
    }

    /// Returns the API endpoint to query the repository's events.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn events_url(&self) -> &Url {
        &self.events_url
    }

    /// Returns the API endpoint to query the repository's assignees.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn assignees_url(&self) -> &Url {
        &self.assignees_url
    }

    /// Returns the API endpoint to query the repository's branches.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn branches_url(&self) -> &Url {
        &self.branches_url
    }

    /// Returns the API endpoint to query the repository's tags.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn tags_url(&self) -> &Url {
        &self.tags_url
    }

    /// Returns the API endpoint to query the repository's blobs.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn blobs_url(&self) -> &Url {
        &self.blobs_url
    }

    /// Returns the API endpoint to query the repository's git tags.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn git_tags_url(&self) -> &Url {
        &self.git_tags_url
    }

    /// Returns the API endpoint to query the repository's git refs.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn git_refs_url(&self) -> &Url {
        &self.git_refs_url
    }

    /// Returns the API endpoint to query the repository's git trees.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn trees_url(&self) -> &Url {
        &self.trees_url
    }

    /// Returns the API endpoint to query the repository's statuses.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn statuses_url(&self) -> &Url {
        &self.statuses_url
    }

    /// Returns the API endpoint to query the repository's programming languages.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn languages_url(&self) -> &Url {
        &self.languages_url
    }

    /// Returns the API endpoint to query the repository's stargazers.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn stargazers_url(&self) -> &Url {
        &self.stargazers_url
    }

    /// Returns the API endpoint to query the repository's contributors.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn contributors_url(&self) -> &Url {
        &self.contributors_url
    }

    /// Returns the API endpoint to query the repository's subscribers.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn subscribers_url(&self) -> &Url {
        &self.subscribers_url
    }

    /// Returns the API endpoint to query the repository's subscriptions.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn subscription_url(&self) -> &Url {
        &self.subscription_url
    }

    /// Returns the API endpoint to query the repository's commits.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn commits_url(&self) -> &Url {
        &self.commits_url
    }

    /// Returns the API endpoint to query the repository's git commits.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn git_commits_url(&self) -> &Url {
        &self.git_commits_url
    }

    /// Returns the API endpoint to query the repository's comments.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn comments_url(&self) -> &Url {
        &self.comments_url
    }

    /// Returns the API endpoint to query the repository's issue comments.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn issue_comment_url(&self) -> &Url {
        &self.issue_comment_url
    }

    /// Returns the API endpoint to query the repository's contents.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn contents_url(&self) -> &Url {
        &self.contents_url
    }

    /// Returns the API endpoint to compare refs in the repository.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn compare_url(&self) -> &Url {
        &self.compare_url
    }

    /// Returns the API endpoint to query the repository's merges.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn merges_url(&self) -> &Url {
        &self.merges_url
    }

    /// Returns the API endpoint to retrieve the repository's archive.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn archive_url(&self) -> &Url {
        &self.archive_url
    }

    /// Returns the API endpoint to query the repository's downloads.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn downloads_url(&self) -> &Url {
        &self.downloads_url
    }

    /// Returns the API endpoint to query the repository's issues.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn issues_url(&self) -> &Url {
        &self.issues_url
    }

    /// Returns the API endpoint to query the repository's pull requests.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn pulls_url(&self) -> &Url {
        &self.pulls_url
    }

    /// Returns the API endpoint to query the repository's milestones.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn milestones_url(&self) -> &Url {
        &self.milestones_url
    }

    /// Returns the API endpoint to query the repository's notifications.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn notifications_url(&self) -> &Url {
        &self.notifications_url
    }

    /// Returns the API endpoint to query the repository's labels.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn labels_url(&self) -> &Url {
        &self.labels_url
    }

    /// Returns the API endpoint to query the repository's releases.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn releases_url(&self) -> &Url {
        &self.releases_url
    }

    /// Returns the API endpoint to query the repository's deployments.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn deployments_url(&self) -> &Url {
        &self.deployments_url
    }

    /// Returns the Git URL to clone the repository.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn git_url(&self) -> &Url {
        &self.git_url
    }

    /// Returns the SSH URL to clone the repository.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn ssh_url(&self) -> &str {
        &self.ssh_url
    }

    /// Returns the HTTP URL to clone the repository.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn clone_url(&self) -> &Url {
        &self.clone_url
    }

    /// Returns the SVN URL to clone the repository.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn svn_url(&self) -> &Url {
        &self.svn_url
    }

    /// Returns the URL to the repository's mirror.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn mirror_url(&self) -> &Option<Url> {
        &self.mirror_url
    }

    /// Returns the date when the repository was created.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    /// Returns the date when the repository was last updated.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    /// Returns the date when the repository was last pushed.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn pushed_at(&self) -> &DateTime<Utc> {
        &self.pushed_at
    }
}

impl Display for Repository {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_name)
    }
}

#[cfg(test)]
mod tests {
    use super::Repository;

    #[test]
    fn trait_deserialize() {
        let repository: Repository = serde_json::from_str(include_str!(
            "../../../tests/fixtures/resource/repository.json"
        ))
        .unwrap();

        assert_eq!("automatons", repository.name().get());
    }

    #[test]
    fn trait_display() {
        let repository: Repository = serde_json::from_str(include_str!(
            "../../../tests/fixtures/resource/repository.json"
        ))
        .unwrap();

        assert_eq!("devxbots/automatons", repository.to_string());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Repository>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Repository>();
    }
}
