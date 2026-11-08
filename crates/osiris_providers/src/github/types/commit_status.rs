//! A status a service set on a commit.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A status a service set on a commit.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubCommitStatus {
    pub id: String,
    pub state: String,
    pub context: String,
    pub description: Option<String>,
    pub target_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
