//! `github.rerun_failed_jobs`: runs again the jobs of a workflow run that failed, and the jobs that depend on them.

use crate::{
    ProviderError,
    github::{Github, path_segment::path_segment, request::send, types::GithubAcknowledgement},
};
use reqwest::Method;
use serde::{Deserialize, Serialize, de::IgnoredAny};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct RerunFailedJobs {
    pub owner: String,
    pub repo: String,
    pub run_id: u64,
    /// Whether the jobs run again with debug logging turned on.
    pub enable_debug_logging: Option<bool>,
}

impl RerunFailedJobs {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["workflow"];
}

/// The re-run's options, as GitHub reads them.
#[derive(Serialize)]
struct Body {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_debug_logging: Option<bool>,
}

impl Github {
    /// `POST /repos/{owner}/{repo}/actions/runs/{run_id}/rerun-failed-jobs`, whose answer carries nothing of
    /// its own.
    pub async fn rerun_failed_jobs(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &RerunFailedJobs,
    ) -> Result<GithubAcknowledgement, ProviderError> {
        let path = format!(
            "/repos/{}/{}/actions/runs/{}/rerun-failed-jobs",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.run_id.to_string())
        );
        let body = Body {
            enable_debug_logging: input.enable_debug_logging,
        };
        let _: IgnoredAny = send(self, http, token, Method::POST, &path, &[], Some(&body)).await?;
        Ok(GithubAcknowledgement { ok: true })
    }
}
