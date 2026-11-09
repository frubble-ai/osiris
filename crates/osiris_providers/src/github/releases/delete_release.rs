//! `github.delete_release`: a release deleted, by its id. Its tag stays.

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
pub struct DeleteRelease {
    /// The account or organization that owns the repository.
    pub owner: String,
    pub repo: String,
    /// GitHub's numeric id of the release.
    pub release_id: GithubId,
}

impl DeleteRelease {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `DELETE /repos/{owner}/{repo}/releases/{release_id}`, which answers nothing.
    pub async fn delete_release(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &DeleteRelease,
    ) -> Result<GithubAcknowledgement, ProviderError> {
        let path = format!(
            "/repos/{}/{}/releases/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.release_id.to_string())
        );
        send::<(), ()>(self, http, token, Method::DELETE, &path, &[], None).await?;
        Ok(GithubAcknowledgement { ok: true })
    }
}
