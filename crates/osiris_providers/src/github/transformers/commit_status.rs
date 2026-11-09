//! A status a service set on a commit, as GitHub answers it.

use super::user::UserAnswer;
use crate::github::types::{GithubCommitStatus, GithubUser};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CommitStatusAnswer {
    pub id: u64,
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub context: String,
    pub description: Option<String>,
    pub target_url: Option<String>,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
    pub creator: Option<UserAnswer>,
}

impl From<CommitStatusAnswer> for GithubCommitStatus {
    /// A status a service set on a commit, in our terms.
    fn from(answer: CommitStatusAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            state: answer.state,
            context: answer.context,
            description: answer.description,
            target_url: answer.target_url,
            created_at: answer.created_at,
            updated_at: answer.updated_at,
            creator: answer.creator.map(GithubUser::from),
        }
    }
}
