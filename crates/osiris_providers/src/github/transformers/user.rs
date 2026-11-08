//! An account, as GitHub answers it inside a resource and at `GET /user`.

use crate::github::types::{GithubCurrentUser, GithubUser};
use serde::Deserialize;

/// An account, as GitHub names one inside a resource.
#[derive(Deserialize)]
pub struct UserAnswer {
    pub id: u64,
    pub login: String,
    #[serde(default)]
    pub avatar_url: String,
    #[serde(default)]
    pub html_url: String,
    #[serde(rename = "type", default)]
    pub kind: String,
}

/// `GET /user`: the authenticated user, with their profile.
#[derive(Deserialize)]
pub struct CurrentUserAnswer {
    #[serde(flatten)]
    pub user: UserAnswer,
    pub name: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub company: Option<String>,
    pub location: Option<String>,
}

impl From<UserAnswer> for GithubUser {
    /// The account in our terms.
    fn from(answer: UserAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            login: answer.login,
            avatar_url: answer.avatar_url,
            url: answer.html_url,
            account_type: answer.kind,
        }
    }
}

impl From<CurrentUserAnswer> for GithubCurrentUser {
    /// The authenticated user in our terms.
    fn from(answer: CurrentUserAnswer) -> Self {
        let user = GithubUser::from(answer.user);
        Self {
            id: user.id,
            login: user.login,
            avatar_url: user.avatar_url,
            url: user.url,
            account_type: user.account_type,
            name: answer.name,
            email: answer.email,
            bio: answer.bio,
            company: answer.company,
            location: answer.location,
        }
    }
}
