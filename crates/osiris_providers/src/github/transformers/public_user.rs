//! A user's public profile, as GitHub answers it.

use crate::github::types::GithubPublicUser;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PublicUserAnswer {
    pub id: u64,
    #[serde(default)]
    pub login: String,
    #[serde(default)]
    pub avatar_url: String,
    #[serde(rename = "html_url", default)]
    pub url: String,
    #[serde(rename = "type", default)]
    pub account_type: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub company: Option<String>,
    pub location: Option<String>,
    #[serde(default)]
    pub followers: i64,
    #[serde(default)]
    pub following: i64,
    #[serde(default)]
    pub public_repos: i64,
}

impl From<PublicUserAnswer> for GithubPublicUser {
    /// A user's public profile, in our terms.
    fn from(answer: PublicUserAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            login: answer.login,
            avatar_url: answer.avatar_url,
            url: answer.url,
            account_type: answer.account_type,
            name: answer.name,
            email: answer.email,
            bio: answer.bio,
            company: answer.company,
            location: answer.location,
            followers: answer.followers,
            following: answer.following,
            public_repos: answer.public_repos,
        }
    }
}
