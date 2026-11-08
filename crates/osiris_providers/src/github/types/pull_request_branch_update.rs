//! A pull request's branch being brought up to date with its base branch.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A pull request's branch being brought up to date with its base branch, which GitHub does in
/// the background.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubPullRequestBranchUpdate {
    /// What GitHub says it is doing.
    pub message: String,
    /// The pull request's address on GitHub's API.
    pub url: String,
}
