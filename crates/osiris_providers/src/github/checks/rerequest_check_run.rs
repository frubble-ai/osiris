//! `github.rerequest_check_run`: asks the app that made a check run to run it again.

use crate::{
    ProviderError,
    github::{Github, path_segment::path_segment, request::send, types::GithubAcknowledgement},
};
use reqwest::Method;
use serde::{Deserialize, de::IgnoredAny};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct RerequestCheckRun {
    pub owner: String,
    pub repo: String,
    pub check_run_id: u64,
}

impl RerequestCheckRun {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["workflow"];
}

impl Github {
    /// `POST /repos/{owner}/{repo}/check-runs/{check_run_id}/rerequest`, whose answer carries
    /// nothing of its own.
    pub async fn rerequest_check_run(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &RerequestCheckRun,
    ) -> Result<GithubAcknowledgement, ProviderError> {
        let path = format!(
            "/repos/{}/{}/check-runs/{}/rerequest",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.check_run_id.to_string())
        );
        let _: IgnoredAny =
            send::<_, ()>(self, http, token, Method::POST, &path, &[], None).await?;
        Ok(GithubAcknowledgement { ok: true })
    }
}
