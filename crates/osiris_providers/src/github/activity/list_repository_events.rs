//! `github.list_repository_events`: a repository's recent events, a page at a time.

use super::list_events::list_events;
use crate::{
    ProviderError,
    github::{Github, path_segment::path_segment, types::GithubEvent},
};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListRepositoryEvents {
    /// The account or organization that owns the repository.
    pub owner: String,
    pub repo: String,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListRepositoryEvents {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/events`.
    pub async fn list_repository_events(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListRepositoryEvents,
    ) -> Result<Vec<GithubEvent>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/events",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        list_events(self, http, token, &path, input.per_page, input.page).await
    }
}
