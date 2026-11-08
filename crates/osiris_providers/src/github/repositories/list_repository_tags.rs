//! `github.list_repository_tags`: a repository's tags, a page at a time.

use crate::{
    ProviderError,
    github::{
        Github, paging::paging, path_segment::path_segment, request::send,
        transformers::tag::TagAnswer, types::GithubTag,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListRepositoryTags {
    pub owner: String,
    pub repo: String,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListRepositoryTags {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/tags`.
    pub async fn list_repository_tags(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListRepositoryTags,
    ) -> Result<Vec<GithubTag>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/tags",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let query = paging(input.per_page, input.page);
        let answer: Vec<TagAnswer> =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer.into_iter().map(GithubTag::from).collect())
    }
}
