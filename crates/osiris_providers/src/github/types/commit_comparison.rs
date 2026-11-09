//! Two commits compared: how far apart they are, and the commits and files between them.

use super::{GithubCommit, GithubCommitFile};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Two commits compared: how far apart they are, and the commits and files between them.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubCommitComparison {
    pub status: String,
    pub ahead_by: i64,
    pub behind_by: i64,
    pub total_commits: i64,
    pub url: String,
    pub commits: Vec<GithubCommit>,
    pub files: Vec<GithubCommitFile>,
    /// The commit compared from, and the latest one both sides share.
    pub base_commit_sha: Option<String>,
    pub merge_base_commit_sha: Option<String>,
}
