//! `github.list_repository_labels`: the labels a repository has, a page at a time.

use crate::{
    ProviderError,
    github::{
        Github, paging::paging, path_segment::path_segment, request::send,
        transformers::label::LabelAnswer, types::GithubLabel,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListRepositoryLabels {
    pub owner: String,
    pub repo: String,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListRepositoryLabels {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/labels`.
    pub async fn list_repository_labels(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListRepositoryLabels,
    ) -> Result<Vec<GithubLabel>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/labels",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let query = paging(input.per_page, input.page);
        let answer: Vec<LabelAnswer> =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer.into_iter().map(GithubLabel::from).collect())
    }
}
