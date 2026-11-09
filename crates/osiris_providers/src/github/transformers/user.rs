//! An account, as GitHub answers it inside a resource and at `GET /user`.

use super::public_user::PublicUserAnswer;
use crate::github::types::{GithubCurrentUser, GithubPublicUser, GithubUser};
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
    pub profile: PublicUserAnswer,
    pub total_private_repos: Option<i64>,
    pub owned_private_repos: Option<i64>,
    pub private_gists: Option<i64>,
    pub disk_usage: Option<i64>,
    pub collaborators: Option<i64>,
    pub two_factor_authentication: Option<bool>,
    pub plan: Option<PlanAnswer>,
}

/// The account's plan, of which we keep the name.
#[derive(Deserialize)]
pub struct PlanAnswer {
    pub name: Option<String>,
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
        Self {
            profile: GithubPublicUser::from(answer.profile),
            total_private_repos: answer.total_private_repos,
            owned_private_repos: answer.owned_private_repos,
            private_gists: answer.private_gists,
            disk_usage: answer.disk_usage,
            collaborators: answer.collaborators,
            two_factor_authentication: answer.two_factor_authentication,
            plan: answer.plan.and_then(|plan| plan.name),
        }
    }
}
