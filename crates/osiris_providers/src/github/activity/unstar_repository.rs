//! `github.unstar_repository`: a repository's star taken back by the authenticated user.

use crate::{
    ProviderError,
    github::{Github, path_segment::path_segment, request::send, types::GithubAcknowledgement},
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct UnstarRepository {
    /// The account or organization that owns the repository.
    pub owner: String,
    pub repo: String,
}

impl UnstarRepository {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `DELETE /user/starred/{owner}/{repo}`, which answers nothing.
    pub async fn unstar_repository(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &UnstarRepository,
    ) -> Result<GithubAcknowledgement, ProviderError> {
        let path = format!(
            "/user/starred/{}/{}",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        send::<(), ()>(self, http, token, Method::DELETE, &path, &[], None).await?;
        Ok(GithubAcknowledgement { ok: true })
    }
}
