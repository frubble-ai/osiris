//! A commit's statuses, and the state they add up to, as GitHub answers it.

use super::commit_status::CommitStatusAnswer;
use crate::github::types::{GithubCombinedStatus, GithubCommitStatus};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CombinedStatusAnswer {
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub sha: String,
    #[serde(default)]
    pub total_count: i64,
    #[serde(default)]
    pub statuses: Vec<CommitStatusAnswer>,
}

impl From<CombinedStatusAnswer> for GithubCombinedStatus {
    /// A commit's statuses, and the state they add up to, in our terms.
    fn from(answer: CombinedStatusAnswer) -> Self {
        Self {
            state: answer.state,
            sha: answer.sha,
            total_count: answer.total_count,
            statuses: answer
                .statuses
                .into_iter()
                .map(GithubCommitStatus::from)
                .collect(),
        }
    }
}
