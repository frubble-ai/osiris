//! `github.list_issue_timeline_events`: everything on an issue's timeline, a page at a time.

use crate::{
    ProviderError,
    github::{
        Github, paging::paging, path_segment::path_segment, request::send,
        transformers::timeline_event::TimelineEventAnswer, types::GithubTimelineEvent,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListIssueTimelineEvents {
    pub owner: String,
    pub repo: String,
    pub issue_number: u64,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListIssueTimelineEvents {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/issues/{issue_number}/timeline`.
    pub async fn list_issue_timeline_events(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListIssueTimelineEvents,
    ) -> Result<Vec<GithubTimelineEvent>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/issues/{}/timeline",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.issue_number.to_string())
        );
        let query = paging(input.per_page, input.page);
        let answer: Vec<TimelineEventAnswer> =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer.into_iter().map(GithubTimelineEvent::from).collect())
    }
}
