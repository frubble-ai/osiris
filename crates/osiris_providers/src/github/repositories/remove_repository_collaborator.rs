//! `github.remove_repository_collaborator`: a collaborator removed from a repository.

use crate::{
    ProviderError,
    github::{Github, path_segment::path_segment, request::send, types::GithubAcknowledgement},
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct RemoveRepositoryCollaborator {
    pub owner: String,
    pub repo: String,
    /// The collaborator's login.
    pub username: String,
}

impl RemoveRepositoryCollaborator {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `DELETE /repos/{owner}/{repo}/collaborators/{username}`.
    pub async fn remove_repository_collaborator(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &RemoveRepositoryCollaborator,
    ) -> Result<GithubAcknowledgement, ProviderError> {
        let path = format!(
            "/repos/{}/{}/collaborators/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.username)
        );
        send::<(), ()>(self, http, token, Method::DELETE, &path, &[], None).await?;
        Ok(GithubAcknowledgement { ok: true })
    }
}
