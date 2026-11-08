//! `github.get_workflow_job_logs`: the plain-text logs of a finished workflow job.

use super::follow_download::follow_download;
use crate::{
    ProviderCommon, ProviderError,
    github::{Github, path_segment::path_segment, types::GithubJobLogs},
};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct GetWorkflowJobLogs {
    pub owner: String,
    pub repo: String,
    pub job_id: u64,
}

impl GetWorkflowJobLogs {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/actions/jobs/{job_id}/logs`, which redirects to the logs as
    /// plain text; the redirect is followed and the whole text read.
    pub async fn get_workflow_job_logs(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &GetWorkflowJobLogs,
    ) -> Result<GithubJobLogs, ProviderError> {
        let path = format!(
            "/repos/{}/{}/actions/jobs/{}/logs",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.job_id.to_string())
        );
        let logs = follow_download(self, http, token, &path)
            .await?
            .text()
            .await
            .map_err(|error| ProviderError::unavailable(self.id(), error.without_url()))?;
        Ok(GithubJobLogs { logs })
    }
}
