//! `github.list_user_repositories`: a user's public repositories, a page at a time.

use crate::{
    ProviderError,
    github::{
        Github, paging::paging, path_segment::path_segment, request::send,
        transformers::repository::RepositoryAnswer, trimmed::trimmed, types::GithubRepository,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListUserRepositories {
    /// The user's login.
    pub username: String,
    /// `all`, `owner` or `member`.
    #[serde(rename = "type")]
    pub repository_type: Option<String>,
    /// `created`, `updated`, `pushed` or `full_name`.
    pub sort: Option<String>,
    /// `asc` or `desc`.
    pub direction: Option<String>,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListUserRepositories {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /users/{username}/repos`.
    pub async fn list_user_repositories(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListUserRepositories,
    ) -> Result<Vec<GithubRepository>, ProviderError> {
        let path = format!("/users/{}/repos", path_segment(&input.username));
        let mut query = Vec::new();
        for (name, value) in [
            ("type", &input.repository_type),
            ("sort", &input.sort),
            ("direction", &input.direction),
        ] {
            if let Some(value) = trimmed(value.as_deref()) {
                query.push((name, value.to_owned()));
            }
        }
        query.extend(paging(input.per_page, input.page));
        let answer: Vec<RepositoryAnswer> =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer.into_iter().map(GithubRepository::from).collect())
    }
}
