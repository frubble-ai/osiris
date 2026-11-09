//! `github.list_workflow_run_artifacts`: the artifacts a workflow run kept, a page at a time.

use crate::{
    ProviderError,
    github::{
        Github, github_id::GithubId, paging::paging, path_segment::path_segment, request::send,
        transformers::artifacts_page::ArtifactsPage, trimmed::trimmed, types::GithubArtifact,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListWorkflowRunArtifacts {
    pub owner: String,
    pub repo: String,
    pub run_id: GithubId,
    /// Only the artifacts with exactly this name.
    pub name: Option<String>,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListWorkflowRunArtifacts {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/actions/runs/{run_id}/artifacts`.
    pub async fn list_workflow_run_artifacts(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListWorkflowRunArtifacts,
    ) -> Result<Vec<GithubArtifact>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/actions/runs/{}/artifacts",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.run_id.to_string())
        );
        let mut query = paging(input.per_page, input.page);
        if let Some(name) = trimmed(input.name.as_deref()) {
            query.push(("name", name.to_owned()));
        }
        let answer: ArtifactsPage =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer
            .artifacts
            .into_iter()
            .map(GithubArtifact::from)
            .collect())
    }
}
