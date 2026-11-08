//! `github.list_pull_request_review_comments`: the comments on a pull request's diff, a page at a
//! time.

use super::SortDirection;
use crate::{
    ProviderError,
    github::{
        Github, paging::paging, path_segment::path_segment, request::send,
        transformers::review_comment::ReviewCommentAnswer, types::GithubReviewComment,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

/// What a list of review comments is sorted by.
#[derive(Debug, Clone, Copy, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReviewCommentSort {
    Created,
    Updated,
}

impl ReviewCommentSort {
    /// The sort as GitHub names it in a query.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Updated => "updated",
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListPullRequestReviewComments {
    pub owner: String,
    pub repo: String,
    pub pull_number: u64,
    pub sort: Option<ReviewCommentSort>,
    pub direction: Option<SortDirection>,
    /// Only comments updated at or after this time, in ISO 8601 (`2024-01-31T12:00:00Z`).
    pub since: Option<String>,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListPullRequestReviewComments {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/pulls/{pull_number}/comments`.
    pub async fn list_pull_request_review_comments(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListPullRequestReviewComments,
    ) -> Result<Vec<GithubReviewComment>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/pulls/{}/comments",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.pull_number.to_string())
        );
        let mut query = paging(input.per_page, input.page);
        if let Some(sort) = input.sort {
            query.push(("sort", sort.as_str().to_owned()));
        }
        if let Some(direction) = input.direction {
            query.push(("direction", direction.as_str().to_owned()));
        }
        if let Some(since) = &input.since {
            query.push(("since", since.clone()));
        }
        let answer: Vec<ReviewCommentAnswer> =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer.into_iter().map(GithubReviewComment::from).collect())
    }
}
