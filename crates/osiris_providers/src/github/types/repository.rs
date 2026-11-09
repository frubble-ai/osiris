//! A repository, and its parts.

use super::{GithubPermissions, GithubUser};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A repository.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "GitHub's own flags, one each"
)]
pub struct GithubRepository {
    pub id: String,
    pub name: String,
    /// `owner/name`.
    pub full_name: String,
    pub private: bool,
    /// Its page on GitHub.
    pub url: String,
    pub clone_url: String,
    pub ssh_url: String,
    pub description: Option<String>,
    pub default_branch: String,
    /// `public`, `private` or `internal` today. An open string.
    pub visibility: String,
    pub fork: bool,
    pub owner: GithubUser,
    pub homepage: Option<String>,
    pub language: Option<String>,
    pub topics: Vec<String>,
    pub license: Option<GithubLicense>,
    pub archived: bool,
    pub disabled: bool,
    pub is_template: bool,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub has_discussions: bool,
    pub stargazers_count: i64,
    pub watchers_count: i64,
    pub forks_count: i64,
    pub open_issues_count: i64,
    /// Its size in kilobytes.
    pub size: i64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub pushed_at: Option<String>,
    /// What the account asking may do in it; GitHub leaves it out where nobody signed in asks.
    pub permissions: Option<GithubPermissions>,
    /// The merge settings and the watcher count, which GitHub answers only when it reads one
    /// repository by itself.
    pub allow_merge_commit: Option<bool>,
    pub allow_squash_merge: Option<bool>,
    pub allow_rebase_merge: Option<bool>,
    pub allow_auto_merge: Option<bool>,
    pub delete_branch_on_merge: Option<bool>,
    pub subscribers_count: Option<i64>,
}

/// The license GitHub found in a repository.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct GithubLicense {
    pub key: String,
    pub name: String,
    pub spdx_id: Option<String>,
}
