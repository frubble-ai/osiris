//! A commit's statuses, and the state they add up to.

use super::GithubCommitStatus;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A commit's statuses, and the state they add up to.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubCombinedStatus {
    pub state: String,
    pub sha: String,
    pub total_count: i64,
    pub statuses: Vec<GithubCommitStatus>,
}
