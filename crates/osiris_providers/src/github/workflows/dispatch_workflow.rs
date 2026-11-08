//! `github.dispatch_workflow`: starts a workflow that runs on `workflow_dispatch`.

use crate::{
    ProviderError,
    github::{Github, path_segment::path_segment, request::send, types::GithubAcknowledgement},
};
use reqwest::Method;
use serde::{Deserialize, Serialize, de::IgnoredAny};
use std::collections::BTreeMap;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct DispatchWorkflow {
    pub owner: String,
    pub repo: String,
    /// The workflow's id, or its file name such as `ci.yml`.
    pub workflow_id: String,
    /// The branch or tag the workflow runs on.
    #[serde(rename = "ref")]
    pub git_ref: String,
    /// The workflow's inputs, by name. Every value is a string.
    pub inputs: Option<BTreeMap<String, String>>,
}

impl DispatchWorkflow {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["workflow"];
}

/// The dispatch, as GitHub reads it.
#[derive(Serialize)]
struct Body<'a> {
    #[serde(rename = "ref")]
    git_ref: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    inputs: Option<&'a BTreeMap<String, String>>,
}

impl Github {
    /// `POST /repos/{owner}/{repo}/actions/workflows/{workflow_id}/dispatches`, which answers
    /// nothing.
    pub async fn dispatch_workflow(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &DispatchWorkflow,
    ) -> Result<GithubAcknowledgement, ProviderError> {
        let path = format!(
            "/repos/{}/{}/actions/workflows/{}/dispatches",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.workflow_id)
        );
        let body = Body {
            git_ref: &input.git_ref,
            inputs: input.inputs.as_ref(),
        };
        let _: IgnoredAny = send(self, http, token, Method::POST, &path, &[], Some(&body)).await?;
        Ok(GithubAcknowledgement { ok: true })
    }
}
