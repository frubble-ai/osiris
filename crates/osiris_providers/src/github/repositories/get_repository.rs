//! `github.get_repository`: a repository, by owner and name.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send,
        transformers::repository::RepositoryAnswer, types::GithubRepository,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct GetRepository {
    /// The account or organization that owns it.
    pub owner: String,
    pub repo: String,
}

impl GetRepository {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}`.
    pub async fn get_repository(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &GetRepository,
    ) -> Result<GithubRepository, ProviderError> {
        let path = format!(
            "/repos/{}/{}",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let answer: RepositoryAnswer =
            send::<_, ()>(self, http, token, Method::GET, &path, &[], None).await?;
        Ok(answer.into())
    }
}
