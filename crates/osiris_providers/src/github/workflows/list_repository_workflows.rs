//! `github.list_repository_workflows`: the workflows a repository has, a page at a time.

use crate::{
    ProviderError,
    github::{
        Github, paging::paging, path_segment::path_segment, request::send,
        transformers::workflows_page::WorkflowsPage, types::GithubWorkflow,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListRepositoryWorkflows {
    pub owner: String,
    pub repo: String,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListRepositoryWorkflows {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/actions/workflows`.
    pub async fn list_repository_workflows(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListRepositoryWorkflows,
    ) -> Result<Vec<GithubWorkflow>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/actions/workflows",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let query = paging(input.per_page, input.page);
        let answer: WorkflowsPage =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer
            .workflows
            .into_iter()
            .map(GithubWorkflow::from)
            .collect())
    }
}
