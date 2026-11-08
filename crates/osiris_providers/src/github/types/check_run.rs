//! A check run on a commit.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A check run on a commit.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubCheckRun {
    pub id: String,
    pub name: String,
    pub status: String,
    pub conclusion: Option<String>,
    pub head_sha: String,
    pub url: Option<String>,
    pub details_url: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
}
