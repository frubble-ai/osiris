//! A file a workflow run kept, as GitHub answers it.

use crate::github::types::GithubArtifact;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ArtifactAnswer {
    pub id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub size_in_bytes: i64,
    #[serde(default)]
    pub archive_download_url: String,
    #[serde(default)]
    pub expired: bool,
    pub created_at: Option<String>,
    pub expires_at: Option<String>,
}

impl From<ArtifactAnswer> for GithubArtifact {
    /// A file a workflow run kept, in our terms.
    fn from(answer: ArtifactAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            name: answer.name,
            size_in_bytes: answer.size_in_bytes,
            archive_download_url: answer.archive_download_url,
            expired: answer.expired,
            created_at: answer.created_at,
            expires_at: answer.expires_at,
        }
    }
}
