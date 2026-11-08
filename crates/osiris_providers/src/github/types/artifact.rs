//! A file a workflow run kept.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A file a workflow run kept.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubArtifact {
    pub id: String,
    pub name: String,
    pub size_in_bytes: i64,
    pub archive_download_url: String,
    pub expired: bool,
    pub created_at: Option<String>,
    pub expires_at: Option<String>,
}
