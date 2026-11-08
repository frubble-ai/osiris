//! `github.get_latest_release`: a repository's latest published release.

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
pub struct GetLatestRelease {
    /// The account or organization that owns the repository.
    pub owner: String,
    pub repo: String,
}

impl GetLatestRelease {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/releases/latest`.
    pub async fn get_latest_release(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &GetLatestRelease,
    ) -> Result<GithubRelease, ProviderError> {
        let path = format!(
            "/repos/{}/{}/releases/latest",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let answer: ReleaseAnswer =
            send::<_, ()>(self, http, token, Method::GET, &path, &[], None).await?;
        Ok(answer.into())
    }
}
