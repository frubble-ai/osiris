//! `github.delete_release_asset`: a file attached to a release deleted, by its id.

use crate::{
    ProviderError,
    github::{
        Github, github_id::GithubId, path_segment::path_segment, request::send,
        types::GithubAcknowledgement,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct DeleteReleaseAsset {
    /// The account or organization that owns the repository.
    pub owner: String,
    pub repo: String,
    /// GitHub's numeric id of the asset.
    pub asset_id: GithubId,
}

impl DeleteReleaseAsset {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `DELETE /repos/{owner}/{repo}/releases/assets/{asset_id}`, which answers nothing.
    pub async fn delete_release_asset(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &DeleteReleaseAsset,
    ) -> Result<GithubAcknowledgement, ProviderError> {
        let path = format!(
            "/repos/{}/{}/releases/assets/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.asset_id.to_string())
        );
        send::<(), ()>(self, http, token, Method::DELETE, &path, &[], None).await?;
        Ok(GithubAcknowledgement { ok: true })
    }
}
