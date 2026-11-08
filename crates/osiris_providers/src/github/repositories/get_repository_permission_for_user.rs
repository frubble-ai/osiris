//! `github.get_repository_permission_for_user`: what an account may do in a repository.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send,
        transformers::repository_permission::RepositoryPermissionAnswer,
        types::GithubRepositoryPermission,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct GetRepositoryPermissionForUser {
    pub owner: String,
    pub repo: String,
    /// The account's login.
    pub username: String,
}

impl GetRepositoryPermissionForUser {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/collaborators/{username}/permission`.
    pub async fn get_repository_permission_for_user(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &GetRepositoryPermissionForUser,
    ) -> Result<GithubRepositoryPermission, ProviderError> {
        let path = format!(
            "/repos/{}/{}/collaborators/{}/permission",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.username)
        );
        let answer: RepositoryPermissionAnswer =
            send::<_, ()>(self, http, token, Method::GET, &path, &[], None).await?;
        Ok(answer.into())
    }
}
