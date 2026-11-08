//! `github.star_repository`: a repository starred by the authenticated user.

use crate::{
    ProviderError,
    github::{Github, path_segment::path_segment, request::send, types::GithubAcknowledgement},
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct StarRepository {
    /// The account or organization that owns the repository.
    pub owner: String,
    pub repo: String,
}

impl StarRepository {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `PUT /user/starred/{owner}/{repo}`, which answers nothing.
    pub async fn star_repository(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &StarRepository,
    ) -> Result<GithubAcknowledgement, ProviderError> {
        let path = format!(
            "/user/starred/{}/{}",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        send::<(), ()>(self, http, token, Method::PUT, &path, &[], None).await?;
        Ok(GithubAcknowledgement { ok: true })
    }
}
