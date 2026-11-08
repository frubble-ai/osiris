//! `github.search_users`: users that match a search, a page at a time.

use crate::{
    ProviderError,
    github::{
        Github,
        paging::paging,
        request::send,
        transformers::{search::SearchUserAnswer, search_results::SearchResultsAnswer},
        types::GithubUserSearchResults,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct SearchUsers {
    /// What to look for, in GitHub's search syntax: words and qualifiers such as `location:berlin`.
    pub query: String,
    /// `followers`, `repositories` or `joined`. Best match when left out.
    pub sort: Option<String>,
    /// `asc` or `desc` (GitHub's default), when `sort` is given.
    pub order: Option<String>,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl SearchUsers {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &[];
}

impl Github {
    /// `GET /search/users`.
    pub async fn search_users(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &SearchUsers,
    ) -> Result<GithubUserSearchResults, ProviderError> {
        let mut query = vec![("q", input.query.clone())];
        if let Some(sort) = &input.sort {
            query.push(("sort", sort.clone()));
        }
        if let Some(order) = &input.order {
            query.push(("order", order.clone()));
        }
        query.extend(paging(input.per_page, input.page));
        let answer: SearchResultsAnswer<SearchUserAnswer> = send::<_, ()>(
            self,
            http,
            token,
            Method::GET,
            "/search/users",
            &query,
            None,
        )
        .await?;
        Ok(answer.into())
    }
}
