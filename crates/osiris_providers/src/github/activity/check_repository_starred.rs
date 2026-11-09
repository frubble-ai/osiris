//! `github.check_repository_starred`: whether the authenticated user has starred a repository.

use crate::{
    ProviderError,
    github::{Github, path_segment::path_segment, request::send, types::GithubStarredCheck},
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CheckRepositoryStarred {
    /// The account or organization that owns the repository.
    pub owner: String,
    pub repo: String,
}

impl CheckRepositoryStarred {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /user/starred/{owner}/{repo}`, which answers 204 when the user has starred it and 404
    /// when they haven't.
    pub async fn check_repository_starred(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &CheckRepositoryStarred,
    ) -> Result<GithubStarredCheck, ProviderError> {
        let path = format!(
            "/user/starred/{}/{}",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        match send::<(), ()>(self, http, token, Method::GET, &path, &[], None).await {
            Ok(()) => Ok(GithubStarredCheck { starred: true }),
            Err(error) if error.is_not_found() => Ok(GithubStarredCheck { starred: false }),
            Err(error) => Err(error),
        }
    }
}
