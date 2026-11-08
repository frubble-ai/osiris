//! `github.get_current_user`: the authenticated user and their profile.

use crate::{
    ProviderError,
    github::{
        Github, request::send, transformers::user::CurrentUserAnswer, types::GithubCurrentUser,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

/// No input: the user is the connection's.
#[derive(Debug, Default, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct GetCurrentUser {}

impl GetCurrentUser {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["read:user", "user:email"];
}

impl Github {
    /// `GET /user`.
    pub async fn get_current_user(
        &self,
        http: &reqwest::Client,
        token: &str,
        _input: &GetCurrentUser,
    ) -> Result<GithubCurrentUser, ProviderError> {
        let answer: CurrentUserAnswer =
            send::<_, ()>(self, http, token, Method::GET, "/user", &[], None).await?;
        Ok(answer.into())
    }
}
