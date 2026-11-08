//! `github.enable_workflow`: turns a disabled workflow back on.

use crate::{
    ProviderError,
    github::{Github, path_segment::path_segment, request::send, types::GithubAcknowledgement},
};
use reqwest::Method;
use serde::{Deserialize, de::IgnoredAny};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct EnableWorkflow {
    pub owner: String,
    pub repo: String,
    /// The workflow's id, or its file name such as `ci.yml`.
    pub workflow_id: String,
}

impl EnableWorkflow {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["workflow"];
}

impl Github {
    /// `PUT /repos/{owner}/{repo}/actions/workflows/{workflow_id}/enable`, which answers nothing.
    pub async fn enable_workflow(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &EnableWorkflow,
    ) -> Result<GithubAcknowledgement, ProviderError> {
        let path = format!(
            "/repos/{}/{}/actions/workflows/{}/enable",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.workflow_id)
        );
        let _: IgnoredAny = send::<_, ()>(self, http, token, Method::PUT, &path, &[], None).await?;
        Ok(GithubAcknowledgement { ok: true })
    }
}
