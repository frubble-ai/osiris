//! A commit, and its parts.

use super::GithubUser;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// How much a commit changed.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubCommitStats {
    pub additions: i64,
    pub deletions: i64,
    pub total: i64,
}

/// A file a commit changed.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubCommitFile {
    pub sha: Option<String>,
    pub filename: String,
    pub previous_filename: Option<String>,
    pub blob_url: Option<String>,
    pub status: String,
    pub additions: i64,
    pub deletions: i64,
    pub changes: i64,
    pub patch: Option<String>,
}

/// A commit.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubCommit {
    pub sha: String,
    pub url: String,
    pub message: String,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub authored_at: Option<String>,
    pub committer_name: Option<String>,
    pub committer_email: Option<String>,
    pub committed_at: Option<String>,
    pub author: Option<GithubUser>,
    pub committer: Option<GithubUser>,
    pub parents: Vec<String>,
    /// What the commit changed, when GitHub answers it: reading one commit gives it; a list of
    /// commits doesn't, and leaves both null.
    pub stats: Option<GithubCommitStats>,
    pub files: Option<Vec<GithubCommitFile>>,
}
