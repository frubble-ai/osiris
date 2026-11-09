//! `github.search_issues_and_pull_requests`: issues and pull requests found with GitHub's search
//! syntax, structured filters, or both.

use super::search_query::search_query;
use crate::{
    ProviderError,
    github::{
        Github, paging::paging, request::send,
        transformers::search_issue_results::SearchIssueResultsAnswer, trimmed::trimmed,
        types::GithubSearchIssueResults,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

/// The search text and the filters are combined: every filter given narrows the search text.
#[derive(Debug, Default, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct SearchIssuesAndPullRequests {
    /// Search text in GitHub's search syntax, such as `is:open label:bug crash`.
    pub query: Option<String>,
    /// The same as `query`, used only when `query` is left out or empty.
    pub q: Option<String>,
    /// With `repo`, only that repository; alone, only that account's repositories.
    pub owner: Option<String>,
    /// A repository name with `owner`, or `owner/name` alone.
    pub repo: Option<String>,
    /// `open`, `closed` or `all` (no filter).
    pub state: Option<String>,
    /// Only items carrying this label.
    pub label: Option<String>,
    /// Only items opened by this login.
    pub author: Option<String>,
    /// Only items assigned to this login.
    pub assignee: Option<String>,
    /// Only items that mention this login.
    pub mentions: Option<String>,
    /// Only items in repositories written mainly in this language.
    pub language: Option<String>,
    /// Only pull requests into this branch.
    pub base_branch: Option<String>,
    /// Only pull requests from this branch.
    pub head_branch: Option<String>,
    /// Only merged pull requests (`true`) or only unmerged ones (`false`).
    pub is_merged: Option<bool>,
    /// Only issues (`issue`) or only pull requests (`pr`).
    #[serde(rename = "type")]
    pub kind: Option<String>,
    /// `comments`, `reactions`, `reactions-+1`, `reactions--1`, `reactions-smile`,
    /// `reactions-thinking_face`, `reactions-heart`, `reactions-tada`, `interactions`, `created`
    /// or `updated`; best match when left out.
    pub sort: Option<String>,
    /// `asc` or `desc` (GitHub's default).
    pub order: Option<String>,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl SearchIssuesAndPullRequests {
    /// The scopes it needs, as `OpenConnector` names them: none, since search reaches public
    /// repositories without any.
    pub const SCOPES: &[&str] = &[];
}

impl Github {
    /// `GET /search/issues`, the search text built from `query` and the filters.
    pub async fn search_issues_and_pull_requests(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &SearchIssuesAndPullRequests,
    ) -> Result<GithubSearchIssueResults, ProviderError> {
        let mut query = vec![("q", search_query(input))];
        if let Some(sort) = trimmed(input.sort.as_deref()) {
            query.push(("sort", sort.to_owned()));
        }
        if let Some(order) = trimmed(input.order.as_deref()) {
            query.push(("order", order.to_owned()));
        }
        query.extend(paging(input.per_page, input.page));
        let answer: SearchIssueResultsAnswer = send::<_, ()>(
            self,
            http,
            token,
            Method::GET,
            "/search/issues",
            &query,
            None,
        )
        .await?;
        Ok(answer.into())
    }
}
