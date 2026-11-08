//! A GitHub account: a user, an organization or a bot, as a resource names it, and the
//! authenticated user in full.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A GitHub account, as another resource names it (an owner, an author, an assignee).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct GithubUser {
    pub id: String,
    pub login: String,
    pub avatar_url: String,
    pub url: String,
    /// `User`, `Organization` or `Bot` today, as GitHub names it. An open string.
    pub account_type: String,
}

/// The authenticated user, with their profile.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct GithubCurrentUser {
    pub id: String,
    pub login: String,
    pub avatar_url: String,
    pub url: String,
    /// `User` or `Bot` today. An open string.
    pub account_type: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub company: Option<String>,
    pub location: Option<String>,
}
