//! A fork's branch brought up to date with its upstream, as GitHub answers it.

use crate::github::types::GithubForkSync;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ForkSyncAnswer {
    #[serde(default)]
    pub message: String,
    pub merge_type: Option<String>,
    pub base_branch: Option<String>,
}

impl From<ForkSyncAnswer> for GithubForkSync {
    /// A fork's branch brought up to date with its upstream, in our terms.
    fn from(answer: ForkSyncAnswer) -> Self {
        Self {
            message: answer.message,
            merge_type: answer.merge_type,
            base_branch: answer.base_branch,
        }
    }
}
