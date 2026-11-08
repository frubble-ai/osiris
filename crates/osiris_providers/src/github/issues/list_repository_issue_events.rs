//! `github.list_repository_issue_events`: what happened to every issue in a repository, a page
//! at a time.

use crate::{
    ProviderError,
    github::{
        Github, paging::paging, path_segment::path_segment, request::send,
        transformers::issue_event::IssueEventAnswer, types::GithubIssueEvent,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListRepositoryIssueEvents {
    pub owner: String,
    pub repo: String,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListRepositoryIssueEvents {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/issues/events`.
    pub async fn list_repository_issue_events(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListRepositoryIssueEvents,
    ) -> Result<Vec<GithubIssueEvent>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/issues/events",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let query = paging(input.per_page, input.page);
        let answer: Vec<IssueEventAnswer> =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer.into_iter().map(GithubIssueEvent::from).collect())
    }
}
