//! `github.delete_label`: a label deleted from a repository.

use crate::{
    ProviderError,
    github::{Github, path_segment::path_segment, request::send, types::GithubAcknowledgement},
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct DeleteLabel {
    pub owner: String,
    pub repo: String,
    pub name: String,
}

impl DeleteLabel {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `DELETE /repos/{owner}/{repo}/labels/{name}`.
    pub async fn delete_label(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &DeleteLabel,
    ) -> Result<GithubAcknowledgement, ProviderError> {
        let path = format!(
            "/repos/{}/{}/labels/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.name)
        );
        send::<(), ()>(self, http, token, Method::DELETE, &path, &[], None).await?;
        Ok(GithubAcknowledgement { ok: true })
    }
}
