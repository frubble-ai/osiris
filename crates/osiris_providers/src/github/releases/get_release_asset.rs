//! `github.get_release_asset`: a file attached to a release, by its id.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send,
        transformers::release_asset::ReleaseAssetAnswer, types::GithubReleaseAsset,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct GetReleaseAsset {
    /// The account or organization that owns the repository.
    pub owner: String,
    pub repo: String,
    /// GitHub's numeric id of the asset.
    pub asset_id: u64,
}

impl GetReleaseAsset {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/releases/assets/{asset_id}`.
    pub async fn get_release_asset(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &GetReleaseAsset,
    ) -> Result<GithubReleaseAsset, ProviderError> {
        let path = format!(
            "/repos/{}/{}/releases/assets/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.asset_id.to_string())
        );
        let answer: ReleaseAssetAnswer =
            send::<_, ()>(self, http, token, Method::GET, &path, &[], None).await?;
        Ok(answer.into())
    }
}
