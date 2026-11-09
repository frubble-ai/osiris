//! A pull request's branch being brought up to date with its base branch, as GitHub answers it.

use crate::github::types::GithubPullRequestBranchUpdate;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PullRequestBranchUpdateAnswer {
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub url: String,
}

impl From<PullRequestBranchUpdateAnswer> for GithubPullRequestBranchUpdate {
    /// A pull request's branch being brought up to date, in our terms.
    fn from(answer: PullRequestBranchUpdateAnswer) -> Self {
        Self {
            message: answer.message,
            url: answer.url,
        }
    }
}
