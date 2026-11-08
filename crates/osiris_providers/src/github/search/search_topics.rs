//! `github.search_topics`: topics that match a search, a page at a time.

use crate::{
    ProviderError,
    github::{
        Github,
        paging::paging,
        request::send,
        transformers::{search::SearchTopicAnswer, search_results::SearchResultsAnswer},
        types::GithubTopicSearchResults,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct SearchTopics {
    /// What to look for, in GitHub's search syntax: words and qualifiers such as `is:featured`.
    pub query: String,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl SearchTopics {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &[];
}

impl Github {
    /// `GET /search/topics`.
    pub async fn search_topics(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &SearchTopics,
    ) -> Result<GithubTopicSearchResults, ProviderError> {
        let mut query = vec![("q", input.query.clone())];
        query.extend(paging(input.per_page, input.page));
        let answer: SearchResultsAnswer<SearchTopicAnswer> = send::<_, ()>(
            self,
            http,
            token,
            Method::GET,
            "/search/topics",
            &query,
            None,
        )
        .await?;
        Ok(answer.into())
    }
}
