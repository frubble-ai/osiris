//! A branch, as GitHub answers it.

use crate::github::types::GithubBranch;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct BranchAnswer {
    name: String,
    commit: BranchCommitAnswer,
    #[serde(default)]
    protected: bool,
}

#[derive(Deserialize)]
struct BranchCommitAnswer {
    sha: String,
}

impl From<BranchAnswer> for GithubBranch {
    /// The branch in our terms.
    fn from(answer: BranchAnswer) -> Self {
        Self {
            name: answer.name,
            commit_sha: answer.commit.sha,
            protected: answer.protected,
        }
    }
}
