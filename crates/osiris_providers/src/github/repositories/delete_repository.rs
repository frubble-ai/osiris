//! `github.delete_repository`: a repository deleted, by owner and name.

use crate::{
    ProviderError,
    github::{Github, path_segment::path_segment, request::send, types::GithubAcknowledgement},
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct DeleteRepository {
    /// The account or organization that owns it.
    pub owner: String,
    pub repo: String,
}

impl DeleteRepository {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["delete_repo"];
}

impl Github {
    /// `DELETE /repos/{owner}/{repo}`.
    pub async fn delete_repository(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &DeleteRepository,
    ) -> Result<GithubAcknowledgement, ProviderError> {
        let path = format!(
            "/repos/{}/{}",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        send::<(), ()>(self, http, token, Method::DELETE, &path, &[], None).await?;
        Ok(GithubAcknowledgement { ok: true })
    }
}
