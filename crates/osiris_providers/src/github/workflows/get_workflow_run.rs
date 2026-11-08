//! `github.get_workflow_run`: a workflow run, by id.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send,
        transformers::workflow_run::WorkflowRunAnswer, types::GithubWorkflowRun,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct GetWorkflowRun {
    pub owner: String,
    pub repo: String,
    pub run_id: u64,
}

impl GetWorkflowRun {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/actions/runs/{run_id}`.
    pub async fn get_workflow_run(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &GetWorkflowRun,
    ) -> Result<GithubWorkflowRun, ProviderError> {
        let path = format!(
            "/repos/{}/{}/actions/runs/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.run_id.to_string())
        );
        let answer: WorkflowRunAnswer =
            send::<_, ()>(self, http, token, Method::GET, &path, &[], None).await?;
        Ok(answer.into())
    }
}
