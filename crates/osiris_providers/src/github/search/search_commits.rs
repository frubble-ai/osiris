//! `github.search_commits`: commits whose message and details match a search, a page at a time.

use crate::{
    ProviderError,
    github::{
        Github,
        paging::paging,
        request::send,
        transformers::{search::SearchCommitAnswer, search_results::SearchResultsAnswer},
        types::GithubCommitSearchResults,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct SearchCommits {
    /// What to look for, in GitHub's search syntax: words and qualifiers such as `repo:octo/hello`.
    pub query: String,
    /// `author-date` or `committer-date`. Best match when left out.
    pub sort: Option<String>,
    /// `asc` or `desc` (GitHub's default), when `sort` is given.
    pub order: Option<String>,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl SearchCommits {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &[];
}

impl Github {
    /// `GET /search/commits`.
    pub async fn search_commits(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &SearchCommits,
    ) -> Result<GithubCommitSearchResults, ProviderError> {
        let mut query = vec![("q", input.query.clone())];
        if let Some(sort) = &input.sort {
            query.push(("sort", sort.clone()));
        }
        if let Some(order) = &input.order {
            query.push(("order", order.clone()));
        }
        query.extend(paging(input.per_page, input.page));
        let answer: SearchResultsAnswer<SearchCommitAnswer> = send::<_, ()>(
            self,
            http,
            token,
            Method::GET,
            "/search/commits",
            &query,
            None,
        )
        .await?;
        Ok(answer.into())
    }
}
