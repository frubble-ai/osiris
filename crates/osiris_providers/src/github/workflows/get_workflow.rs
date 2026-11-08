//! `github.get_workflow`: a workflow, by id or file name.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send, transformers::workflow::WorkflowAnswer,
        types::GithubWorkflow,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct GetWorkflow {
    pub owner: String,
    pub repo: String,
    /// The workflow's id, or its file name such as `ci.yml`.
    pub workflow_id: String,
}

impl GetWorkflow {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/actions/workflows/{workflow_id}`.
    pub async fn get_workflow(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &GetWorkflow,
    ) -> Result<GithubWorkflow, ProviderError> {
        let path = format!(
            "/repos/{}/{}/actions/workflows/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.workflow_id)
        );
        let answer: WorkflowAnswer =
            send::<_, ()>(self, http, token, Method::GET, &path, &[], None).await?;
        Ok(answer.into())
    }
}
