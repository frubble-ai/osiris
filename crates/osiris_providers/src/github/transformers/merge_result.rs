//! A pull request merged, or a merge refused, as GitHub answers it.

use crate::github::types::GithubMergeResult;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MergeResultAnswer {
    pub sha: Option<String>,
    #[serde(default)]
    pub merged: bool,
    #[serde(default)]
    pub message: String,
}

impl From<MergeResultAnswer> for GithubMergeResult {
    /// A pull request merged, or a merge refused, in our terms.
    fn from(answer: MergeResultAnswer) -> Self {
        Self {
            sha: answer.sha,
            merged: answer.merged,
            message: answer.message,
        }
    }
}
