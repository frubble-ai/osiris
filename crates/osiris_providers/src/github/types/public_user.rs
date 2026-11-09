//! A user's public profile.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A user's public profile.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubPublicUser {
    pub id: String,
    pub login: String,
    pub avatar_url: String,
    pub url: String,
    pub account_type: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub company: Option<String>,
    pub location: Option<String>,
    pub followers: i64,
    pub following: i64,
    pub public_repos: i64,
    pub public_gists: i64,
    pub blog: Option<String>,
    pub twitter_username: Option<String>,
    pub hireable: Option<bool>,
    pub site_admin: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
