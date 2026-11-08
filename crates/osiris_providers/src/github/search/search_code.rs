//! `github.search_code`: files whose code matches a search, a page at a time.

use crate::{
    ProviderError,
    github::{
        Github,
        paging::paging,
        request::send,
        transformers::{search::SearchCodeAnswer, search_results::SearchResultsAnswer},
        types::GithubCodeSearchResults,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct SearchCode {
    /// What to look for, in GitHub's search syntax: words and qualifiers such as `language:rust`.
    pub query: String,
    /// `indexed` (when GitHub last indexed the file) or `updated`. Best match when left out.
    pub sort: Option<String>,
    /// `asc` or `desc` (GitHub's default), when `sort` is given.
    pub order: Option<String>,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl SearchCode {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &[];
}

impl Github {
    /// `GET /search/code`.
    pub async fn search_code(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &SearchCode,
    ) -> Result<GithubCodeSearchResults, ProviderError> {
        let mut query = vec![("q", input.query.clone())];
        if let Some(sort) = &input.sort {
            query.push(("sort", sort.clone()));
        }
        if let Some(order) = &input.order {
            query.push(("order", order.clone()));
        }
        query.extend(paging(input.per_page, input.page));
        let answer: SearchResultsAnswer<SearchCodeAnswer> =
            send::<_, ()>(self, http, token, Method::GET, "/search/code", &query, None).await?;
        Ok(answer.into())
    }
}
