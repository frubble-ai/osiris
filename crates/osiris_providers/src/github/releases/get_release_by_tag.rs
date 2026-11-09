//! `github.get_release_by_tag`: a release, by its tag's name.

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
pub struct GetReleaseByTag {
    /// The account or organization that owns the repository.
    pub owner: String,
    pub repo: String,
    /// The tag's name, such as `v1.0.0`.
    pub tag: String,
}

impl GetReleaseByTag {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/releases/tags/{tag}`.
    pub async fn get_release_by_tag(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &GetReleaseByTag,
    ) -> Result<GithubRelease, ProviderError> {
        let path = format!(
            "/repos/{}/{}/releases/tags/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.tag)
        );
        let answer: ReleaseAnswer =
            send::<_, ()>(self, http, token, Method::GET, &path, &[], None).await?;
        Ok(answer.into())
    }
}
