//! `github.list_workflow_run_jobs`: the jobs of a workflow run, a page at a time.

use crate::{
    ProviderError,
    github::{
        Github, paging::paging, path_segment::path_segment, request::send,
        transformers::workflow_jobs_page::WorkflowJobsPage, trimmed::trimmed,
        types::GithubWorkflowJob,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListWorkflowRunJobs {
    pub owner: String,
    pub repo: String,
    pub run_id: u64,
    /// `latest` (the jobs of the run's latest attempt, GitHub's default) or `all`.
    pub filter: Option<String>,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListWorkflowRunJobs {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/actions/runs/{run_id}/jobs`.
    pub async fn list_workflow_run_jobs(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListWorkflowRunJobs,
    ) -> Result<Vec<GithubWorkflowJob>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/actions/runs/{}/jobs",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.run_id.to_string())
        );
        let mut query = paging(input.per_page, input.page);
        if let Some(filter) = trimmed(input.filter.as_deref()) {
            query.push(("filter", filter.to_owned()));
        }
        let answer: WorkflowJobsPage =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer
            .jobs
            .into_iter()
            .map(GithubWorkflowJob::from)
            .collect())
    }
}
