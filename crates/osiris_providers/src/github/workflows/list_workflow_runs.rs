//! `github.list_workflow_runs`: a repository's workflow runs, newest first, a page at a time.

use crate::{
    ProviderError,
    github::{
        Github, github_id::GithubId, paging::paging, path_segment::path_segment, request::send,
        transformers::workflow_runs_page::WorkflowRunsPage, trimmed::trimmed,
        types::GithubWorkflowRun,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListWorkflowRuns {
    pub owner: String,
    pub repo: String,
    /// Only the runs started by this user's login.
    pub actor: Option<String>,
    /// Only the runs on this branch.
    pub branch: Option<String>,
    /// Only the runs created in this date range, in GitHub's search syntax, such as
    /// `>=2024-01-01` or `2024-01-01..2024-01-31`.
    pub created: Option<String>,
    /// Only the runs of this check suite.
    pub check_suite_id: Option<GithubId>,
    /// Only the runs started by this event, such as `push` or `pull_request`.
    pub event: Option<String>,
    /// Only the runs of this commit.
    pub head_sha: Option<String>,
    /// Only the runs in this status or with this conclusion, such as `in_progress` or `failure`.
    pub status: Option<String>,
    /// Whether to leave out the pull requests GitHub would otherwise list with each run.
    pub exclude_pull_requests: Option<bool>,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListWorkflowRuns {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/actions/runs`.
    pub async fn list_workflow_runs(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListWorkflowRuns,
    ) -> Result<Vec<GithubWorkflowRun>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/actions/runs",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let mut query = paging(input.per_page, input.page);
        let text = |value: &Option<String>| trimmed(value.as_deref()).map(str::to_owned);
        let filters = [
            ("actor", text(&input.actor)),
            ("branch", text(&input.branch)),
            ("created", text(&input.created)),
            (
                "check_suite_id",
                input.check_suite_id.map(|id| id.to_string()),
            ),
            ("event", text(&input.event)),
            ("head_sha", text(&input.head_sha)),
            ("status", text(&input.status)),
            (
                "exclude_pull_requests",
                input
                    .exclude_pull_requests
                    .map(|exclude| exclude.to_string()),
            ),
        ];
        query.extend(
            filters
                .into_iter()
                .filter_map(|(name, value)| value.map(|value| (name, value))),
        );
        let answer: WorkflowRunsPage =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer
            .workflow_runs
            .into_iter()
            .map(GithubWorkflowRun::from)
            .collect())
    }
}
