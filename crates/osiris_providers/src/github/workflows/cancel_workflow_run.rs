//! `github.cancel_workflow_run`: stops a workflow run that is queued or in progress.

use crate::{
    ProviderError,
    github::{
        Github, github_id::GithubId, path_segment::path_segment, request::send,
        types::GithubAcknowledgement,
    },
};
use reqwest::Method;
use serde::{Deserialize, de::IgnoredAny};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CancelWorkflowRun {
    pub owner: String,
    pub repo: String,
    pub run_id: GithubId,
}

impl CancelWorkflowRun {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["workflow"];
}

impl Github {
    /// `POST /repos/{owner}/{repo}/actions/runs/{run_id}/cancel`, whose answer carries nothing of
    /// its own.
    pub async fn cancel_workflow_run(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &CancelWorkflowRun,
    ) -> Result<GithubAcknowledgement, ProviderError> {
        let path = format!(
            "/repos/{}/{}/actions/runs/{}/cancel",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.run_id.to_string())
        );
        let _: IgnoredAny =
            send::<_, ()>(self, http, token, Method::POST, &path, &[], None).await?;
        Ok(GithubAcknowledgement { ok: true })
    }
}
