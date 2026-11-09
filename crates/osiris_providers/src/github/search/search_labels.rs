//! `github.search_labels`: a repository's labels whose name or description match a search, a page
//! at a time.

use crate::{
    ProviderError,
    github::{
        Github,
        github_id::GithubId,
        paging::paging,
        request::send,
        transformers::{search::SearchLabelAnswer, search_results::SearchResultsAnswer},
        trimmed::trimmed,
        types::GithubLabelSearchResults,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct SearchLabels {
    /// GitHub's numeric id of the repository whose labels are searched.
    pub repository_id: GithubId,
    /// The words to look for in the labels' names and descriptions.
    pub query: String,
    /// `created` or `updated`. Best match when left out.
    pub sort: Option<String>,
    /// `asc` or `desc` (GitHub's default), when `sort` is given.
    pub order: Option<String>,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl SearchLabels {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /search/labels`.
    pub async fn search_labels(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &SearchLabels,
    ) -> Result<GithubLabelSearchResults, ProviderError> {
        let mut query = vec![
            ("repository_id", input.repository_id.to_string()),
            ("q", input.query.clone()),
        ];
        if let Some(sort) = trimmed(input.sort.as_deref()) {
            query.push(("sort", sort.to_owned()));
        }
        if let Some(order) = trimmed(input.order.as_deref()) {
            query.push(("order", order.to_owned()));
        }
        query.extend(paging(input.per_page, input.page));
        let answer: SearchResultsAnswer<SearchLabelAnswer> = send::<_, ()>(
            self,
            http,
            token,
            Method::GET,
            "/search/labels",
            &query,
            None,
        )
        .await?;
        Ok(answer.into())
    }
}
