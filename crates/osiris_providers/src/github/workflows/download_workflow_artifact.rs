//! `github.download_workflow_artifact`: where an artifact's ZIP archive can be downloaded.

use super::follow_download::follow_download;
use crate::{
    ProviderCommon, ProviderError,
    github::{
        Github, github_id::GithubId, path_segment::path_segment, types::GithubArtifactDownload,
    },
};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct DownloadWorkflowArtifact {
    pub owner: String,
    pub repo: String,
    pub artifact_id: GithubId,
}

impl DownloadWorkflowArtifact {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/actions/artifacts/{artifact_id}/zip`, which redirects to a
    /// signed URL of the archive. The answer is that URL, for the caller to download: the archive
    /// is a file, and an action answers JSON. The redirect is followed only as far as the
    /// archive's headers, and the archive itself is not read.
    pub async fn download_workflow_artifact(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &DownloadWorkflowArtifact,
    ) -> Result<GithubArtifactDownload, ProviderError> {
        let path = format!(
            "/repos/{}/{}/actions/artifacts/{}/zip",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.artifact_id.to_string())
        );
        let response = follow_download(self, http, token, &path).await?;
        let url = response.url();
        if url.path() == path {
            return Err(ProviderError::unavailable(
                self.id(),
                "GitHub answered the archive itself instead of redirecting to it",
            ));
        }
        Ok(GithubArtifactDownload {
            url: url.to_string(),
        })
    }
}
