//! `github.get_release`: a release, by its id.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send, transformers::release::ReleaseAnswer,
        types::GithubRelease,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct GetRelease {
    /// The account or organization that owns the repository.
    pub owner: String,
    pub repo: String,
    /// GitHub's numeric id of the release.
    pub release_id: u64,
}

impl GetRelease {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/releases/{release_id}`.
    pub async fn get_release(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &GetRelease,
    ) -> Result<GithubRelease, ProviderError> {
        let path = format!(
            "/repos/{}/{}/releases/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.release_id.to_string())
        );
        let answer: ReleaseAnswer =
            send::<_, ()>(self, http, token, Method::GET, &path, &[], None).await?;
        Ok(answer.into())
    }
}
