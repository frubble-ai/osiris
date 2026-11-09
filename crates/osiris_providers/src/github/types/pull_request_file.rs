//! A file a pull request changes.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A file a pull request changes.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubPullRequestFile {
    pub sha: String,
    pub filename: String,
    pub status: String,
    pub additions: i64,
    pub deletions: i64,
    pub changes: i64,
    pub patch: Option<String>,
    pub previous_filename: Option<String>,
    pub blob_url: Option<String>,
}
