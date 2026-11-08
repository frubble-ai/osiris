//! `github.list_pull_requests`: a repository's pull requests, a page at a time.

use super::SortDirection;
use crate::{
    ProviderError,
    github::{
        Github, paging::paging, path_segment::path_segment, request::send,
        transformers::pull_request::PullRequestAnswer, types::GithubPullRequest,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

/// Which pull requests a list holds, by state.
#[derive(Debug, Clone, Copy, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PullRequestStateFilter {
    Open,
    Closed,
    All,
}

impl PullRequestStateFilter {
    /// The state as GitHub names it in a query.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Closed => "closed",
            Self::All => "all",
        }
    }
}

/// What a list of pull requests is sorted by.
#[derive(Debug, Clone, Copy, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum PullRequestSort {
    Created,
    Updated,
    Popularity,
    LongRunning,
}

impl PullRequestSort {
    /// The sort as GitHub names it in a query.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Updated => "updated",
            Self::Popularity => "popularity",
            Self::LongRunning => "long-running",
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListPullRequests {
    pub owner: String,
    pub repo: String,
    /// Open pull requests when left out.
    pub state: Option<PullRequestStateFilter>,
    /// Only pull requests from this branch, as `user:ref-name` or `organization:ref-name`.
    pub head: Option<String>,
    /// Only pull requests into this branch.
    pub base: Option<String>,
    pub sort: Option<PullRequestSort>,
    pub direction: Option<SortDirection>,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListPullRequests {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/pulls`.
    pub async fn list_pull_requests(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListPullRequests,
    ) -> Result<Vec<GithubPullRequest>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/pulls",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let mut query = paging(input.per_page, input.page);
        if let Some(state) = input.state {
            query.push(("state", state.as_str().to_owned()));
        }
        if let Some(head) = &input.head {
            query.push(("head", head.clone()));
        }
        if let Some(base) = &input.base {
            query.push(("base", base.clone()));
        }
        if let Some(sort) = input.sort {
            query.push(("sort", sort.as_str().to_owned()));
        }
        if let Some(direction) = input.direction {
            query.push(("direction", direction.as_str().to_owned()));
        }
        let answer: Vec<PullRequestAnswer> =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer.into_iter().map(GithubPullRequest::from).collect())
    }
}
