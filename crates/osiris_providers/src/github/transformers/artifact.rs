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
    pub updated_at: Option<String>,
    pub digest: Option<String>,
    pub workflow_run: Option<ArtifactRunAnswer>,
}

/// The run that made an artifact.
#[derive(Deserialize)]
pub struct ArtifactRunAnswer {
    pub id: Option<u64>,
    pub head_branch: Option<String>,
    pub head_sha: Option<String>,
}

impl From<ArtifactAnswer> for GithubArtifact {
    /// A file a workflow run kept, in our terms.
    fn from(answer: ArtifactAnswer) -> Self {
        let run = answer.workflow_run.as_ref();
        Self {
            id: answer.id.to_string(),
            name: answer.name,
            size_in_bytes: answer.size_in_bytes,
            archive_download_url: answer.archive_download_url,
            expired: answer.expired,
            created_at: answer.created_at,
            expires_at: answer.expires_at,
            updated_at: answer.updated_at,
            digest: answer.digest,
            run_id: run.and_then(|run| run.id).map(|id| id.to_string()),
            head_branch: run.and_then(|run| run.head_branch.clone()),
            head_sha: run.and_then(|run| run.head_sha.clone()),
        }
    }
}
