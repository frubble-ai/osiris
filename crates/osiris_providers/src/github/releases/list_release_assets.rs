//! `github.list_release_assets`: the files attached to a release, a page at a time.

use crate::{
    ProviderError,
    github::{
        Github, github_id::GithubId, paging::paging, path_segment::path_segment, request::send,
        transformers::release_asset::ReleaseAssetAnswer, types::GithubReleaseAsset,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListReleaseAssets {
    /// The account or organization that owns the repository.
    pub owner: String,
    pub repo: String,
    /// GitHub's numeric id of the release.
    pub release_id: GithubId,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListReleaseAssets {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/releases/{release_id}/assets`.
    pub async fn list_release_assets(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListReleaseAssets,
    ) -> Result<Vec<GithubReleaseAsset>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/releases/{}/assets",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.release_id.to_string())
        );
        let query = paging(input.per_page, input.page);
        let answer: Vec<ReleaseAssetAnswer> =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer.into_iter().map(GithubReleaseAsset::from).collect())
    }
}
