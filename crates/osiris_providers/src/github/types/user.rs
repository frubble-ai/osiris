//! A GitHub account: a user, an organization or a bot, as a resource names it, and the
//! authenticated user in full.

use super::GithubPublicUser;
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

/// The authenticated user: their profile, and what GitHub shows only to the account itself, which
/// is null where the token may not see it.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubCurrentUser {
    #[serde(flatten)]
    pub profile: GithubPublicUser,
    pub total_private_repos: Option<i64>,
    pub owned_private_repos: Option<i64>,
    pub private_gists: Option<i64>,
    /// In kilobytes.
    pub disk_usage: Option<i64>,
    pub collaborators: Option<i64>,
    pub two_factor_authentication: Option<bool>,
    /// The name of the account's plan.
    pub plan: Option<String>,
}
