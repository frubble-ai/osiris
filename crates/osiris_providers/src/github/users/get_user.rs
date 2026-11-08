//! `github.get_user`: a user's public profile, by login.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send,
        transformers::public_user::PublicUserAnswer, types::GithubPublicUser,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct GetUser {
    /// The user's login.
    pub username: String,
}

impl GetUser {
    /// The scopes it needs, as `OpenConnector` names them: none, a profile is public.
    pub const SCOPES: &[&str] = &[];
}

impl Github {
    /// `GET /users/{username}`.
    pub async fn get_user(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &GetUser,
    ) -> Result<GithubPublicUser, ProviderError> {
        let path = format!("/users/{}", path_segment(&input.username));
        let answer: PublicUserAnswer =
            send::<_, ()>(self, http, token, Method::GET, &path, &[], None).await?;
        Ok(answer.into())
    }
}
