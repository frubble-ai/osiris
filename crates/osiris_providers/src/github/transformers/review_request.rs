//! A pull request after reviewers were requested from it or removed from it, as GitHub answers it:
//! the pull request itself, which names who its review is requested from.

use super::{pull_request::PullRequestAnswer, team::TeamAnswer, user::UserAnswer};
use crate::github::types::{GithubPullRequest, GithubReviewRequest, GithubTeam, GithubUser};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ReviewRequestAnswer {
    #[serde(flatten)]
    pub pull_request: PullRequestAnswer,
    #[serde(default)]
    pub requested_reviewers: Vec<UserAnswer>,
    #[serde(default)]
    pub requested_teams: Vec<TeamAnswer>,
}

impl From<ReviewRequestAnswer> for GithubReviewRequest {
    /// A pull request and who its review is requested from, in our terms.
    fn from(answer: ReviewRequestAnswer) -> Self {
        Self {
            pull_request: GithubPullRequest::from(answer.pull_request),
            requested_reviewers: answer
                .requested_reviewers
                .into_iter()
                .map(GithubUser::from)
                .collect(),
            requested_teams: answer
                .requested_teams
                .into_iter()
                .map(GithubTeam::from)
                .collect(),
        }
    }
}
