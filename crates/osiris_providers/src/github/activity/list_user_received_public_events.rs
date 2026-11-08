//! `github.list_user_received_public_events`: the public events a user received from the accounts and repositories they follow and watch, a page at a time.

use super::list_events::list_events;
use crate::{
    ProviderError,
    github::{Github, path_segment::path_segment, types::GithubEvent},
};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListUserReceivedPublicEvents {
    /// The user's login.
    pub username: String,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListUserReceivedPublicEvents {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["read:user", "user:email"];
}

impl Github {
    /// `GET /users/{username}/received_events/public`.
    pub async fn list_user_received_public_events(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListUserReceivedPublicEvents,
    ) -> Result<Vec<GithubEvent>, ProviderError> {
        let path = format!(
            "/users/{}/received_events/public",
            path_segment(&input.username)
        );
        list_events(self, http, token, &path, input.per_page, input.page).await
    }
}
