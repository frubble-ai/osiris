//! `github.list_repository_issues`: a repository's issues, a page at a time, without the pull
//! requests GitHub lists among them.

use crate::{
    ProviderError,
    github::{
        Github,
        paging::paging,
        path_segment::path_segment,
        request::send,
        transformers::issue::IssueAnswer,
        types::{GithubIssue, GithubIssuePage},
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

/// GitHub's page size when none is asked for, a fact of its API. It is always sent, so that
/// `fetched` can be compared with the page size the caller relies on.
const GITHUB_DEFAULT_PER_PAGE: u32 = 30;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListRepositoryIssues {
    pub owner: String,
    pub repo: String,
    /// `open` (GitHub's default), `closed` or `all`.
    pub state: Option<String>,
    /// Only issues carrying every one of these labels.
    pub labels: Option<Vec<String>>,
    /// `created` (GitHub's default), `updated` or `comments`.
    pub sort: Option<String>,
    /// `asc` or `desc` (GitHub's default).
    pub direction: Option<String>,
    /// Only issues updated at or after this time, as an ISO 8601 timestamp.
    pub since: Option<String>,
    /// At most 100; 30 when left out.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListRepositoryIssues {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/issues`, with the pull requests on the page left out and the
    /// page's full length kept as `fetched`.
    pub async fn list_repository_issues(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListRepositoryIssues,
    ) -> Result<GithubIssuePage, ProviderError> {
        let path = format!(
            "/repos/{}/{}/issues",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let mut query = paging(
            Some(input.per_page.unwrap_or(GITHUB_DEFAULT_PER_PAGE)),
            input.page,
        );
        if let Some(state) = &input.state {
            query.push(("state", state.clone()));
        }
        if let Some(labels) = &input.labels {
            query.push(("labels", labels.join(",")));
        }
        if let Some(sort) = &input.sort {
            query.push(("sort", sort.clone()));
        }
        if let Some(direction) = &input.direction {
            query.push(("direction", direction.clone()));
        }
        if let Some(since) = &input.since {
            query.push(("since", since.clone()));
        }
        let answer: Vec<IssueAnswer> =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        let fetched = u64::try_from(answer.len()).unwrap_or(u64::MAX);
        let issues = answer
            .into_iter()
            .map(GithubIssue::from)
            .filter(|issue| !issue.is_pull_request)
            .collect();
        Ok(GithubIssuePage { issues, fetched })
    }
}
