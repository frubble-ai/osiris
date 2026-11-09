//! A topic a search found, and its parts.

use super::{GithubCommit, GithubIssue, GithubRepository, GithubUser};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A repository a search found.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubSearchRepository {
    #[serde(flatten)]
    pub repository: GithubRepository,
    /// How well it matched, as GitHub ranks it.
    pub score: f64,
}

/// An issue or a pull request a search found.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubSearchIssue {
    #[serde(flatten)]
    pub issue: GithubIssue,
    /// The repository it is in, as GitHub's API names it.
    pub repository_url: String,
    /// How well it matched, as GitHub ranks it.
    pub score: f64,
}

/// A user a search found.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubSearchUser {
    pub id: String,
    pub login: String,
    pub account_type: String,
    pub url: String,
    pub avatar_url: String,
    pub score: f64,
}

/// The repository a found commit is in.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubSearchCommitRepository {
    pub id: String,
    pub full_name: String,
    pub url: String,
}

/// A commit a search found.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubSearchCommit {
    #[serde(flatten)]
    pub commit: GithubCommit,
    pub repository: GithubSearchCommitRepository,
    /// How well it matched, as GitHub ranks it.
    pub score: f64,
}

/// The repository found code is in.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubSearchCodeRepository {
    pub id: String,
    pub full_name: String,
    pub url: String,
    pub owner: Option<GithubUser>,
}

/// A file a code search found.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubSearchCode {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub url: String,
    pub git_url: String,
    pub repository: GithubSearchCodeRepository,
    /// How well it matched, as GitHub ranks it.
    pub score: f64,
}

/// A label a search found.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubSearchLabel {
    pub id: String,
    pub name: String,
    pub color: String,
    pub description: Option<String>,
    /// Whether it is one of the labels GitHub gives every new repository.
    pub default: bool,
    pub score: f64,
}

/// A topic a search found.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubSearchTopic {
    pub name: String,
    pub display_name: Option<String>,
    pub short_description: Option<String>,
    pub description: Option<String>,
    pub featured: bool,
    pub curated: bool,
    pub created_by: Option<String>,
    pub released: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub score: f64,
}
