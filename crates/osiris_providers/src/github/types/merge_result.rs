//! A pull request merged, or a merge refused.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A pull request merged, or a merge refused.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubMergeResult {
    pub sha: Option<String>,
    pub merged: bool,
    pub message: String,
}
