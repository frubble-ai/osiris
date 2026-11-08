//! Who a pull request's review is requested from, as GitHub answers it.

use super::team::TeamAnswer;
use super::user::UserAnswer;
use crate::github::types::{GithubRequestedReviewers, GithubTeam, GithubUser};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestedReviewersAnswer {
    #[serde(default)]
    pub users: Vec<UserAnswer>,
    #[serde(default)]
    pub teams: Vec<TeamAnswer>,
}

impl From<RequestedReviewersAnswer> for GithubRequestedReviewers {
    /// Who a pull request's review is requested from, in our terms.
    fn from(answer: RequestedReviewersAnswer) -> Self {
        Self {
            users: answer.users.into_iter().map(GithubUser::from).collect(),
            teams: answer.teams.into_iter().map(GithubTeam::from).collect(),
        }
    }
}
