//! `github.list_public_events`: GitHub's global feed of public events, a page at a time.

use super::list_events::list_events;
use crate::{
    ProviderError,
    github::{Github, types::GithubEvent},
};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Default, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListPublicEvents {
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListPublicEvents {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["read:user", "user:email"];
}

impl Github {
    /// `GET /events`.
    pub async fn list_public_events(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListPublicEvents,
    ) -> Result<Vec<GithubEvent>, ProviderError> {
        list_events(self, http, token, "/events", input.per_page, input.page).await
    }
}
