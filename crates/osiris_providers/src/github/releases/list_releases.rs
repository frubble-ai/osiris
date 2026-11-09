//! `github.list_releases`: a repository's releases, a page at a time.

use crate::{
    ProviderError,
    github::{
        Github, paging::paging, path_segment::path_segment, request::send,
        transformers::release::ReleaseAnswer, types::GithubRelease,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListReleases {
    /// The account or organization that owns the repository.
    pub owner: String,
    pub repo: String,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListReleases {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/releases`.
    pub async fn list_releases(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListReleases,
    ) -> Result<Vec<GithubRelease>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/releases",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let query = paging(input.per_page, input.page);
        let answer: Vec<ReleaseAnswer> =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer.into_iter().map(GithubRelease::from).collect())
    }
}
