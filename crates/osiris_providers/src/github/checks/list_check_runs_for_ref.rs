//! `github.list_check_runs_for_ref`: the check runs on a commit, a branch or a tag, a page at a
//! time.

use crate::{
    ProviderError,
    github::{
        Github, paging::paging, path_segment::path_segment, request::send,
        transformers::check_runs_page::CheckRunsPage, types::GithubCheckRun,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListCheckRunsForRef {
    pub owner: String,
    pub repo: String,
    /// A commit SHA, a branch name or a tag name.
    #[serde(rename = "ref")]
    pub git_ref: String,
    /// Only the check runs of the GitHub App with this id.
    pub app_id: Option<u64>,
    /// Only the check runs with this name.
    pub check_name: Option<String>,
    /// `latest` (each check's most recent run, GitHub's default) or `all`.
    pub filter: Option<String>,
    /// Only the check runs in this status, such as `queued`, `in_progress` or `completed`.
    pub status: Option<String>,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListCheckRunsForRef {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/commits/{ref}/check-runs`.
    pub async fn list_check_runs_for_ref(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListCheckRunsForRef,
    ) -> Result<Vec<GithubCheckRun>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/commits/{}/check-runs",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.git_ref)
        );
        let mut query = paging(input.per_page, input.page);
        if let Some(app_id) = input.app_id {
            query.push(("app_id", app_id.to_string()));
        }
        if let Some(check_name) = &input.check_name {
            query.push(("check_name", check_name.clone()));
        }
        if let Some(filter) = &input.filter {
            query.push(("filter", filter.clone()));
        }
        if let Some(status) = &input.status {
            query.push(("status", status.clone()));
        }
        let answer: CheckRunsPage =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer
            .check_runs
            .into_iter()
            .map(GithubCheckRun::from)
            .collect())
    }
}
