//! `github.list_authenticated_user_events`: the events a user performed, private ones too when the user is the connection's, a page at a time.

use super::list_events::list_events;
use crate::{
    ProviderError,
    github::{Github, path_segment::path_segment, types::GithubEvent},
};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListAuthenticatedUserEvents {
    /// The user's login.
    pub username: String,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListAuthenticatedUserEvents {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["read:user", "user:email"];
}

impl Github {
    /// `GET /users/{username}/events`.
    pub async fn list_authenticated_user_events(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListAuthenticatedUserEvents,
    ) -> Result<Vec<GithubEvent>, ProviderError> {
        let path = format!("/users/{}/events", path_segment(&input.username));
        list_events(self, http, token, &path, input.per_page, input.page).await
    }
}
