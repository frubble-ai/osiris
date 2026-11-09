//! Two commits compared: how far apart they are, and the commits and files between them, as GitHub answers it.

use super::commit::{CommitAnswer, CommitFileAnswer, ShaAnswer};
use crate::github::types::{GithubCommit, GithubCommitComparison, GithubCommitFile};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CommitComparisonAnswer {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub ahead_by: i64,
    #[serde(default)]
    pub behind_by: i64,
    #[serde(default)]
    pub total_commits: i64,
    #[serde(rename = "html_url", default)]
    pub url: String,
    #[serde(default)]
    pub commits: Vec<CommitAnswer>,
    #[serde(default)]
    pub files: Vec<CommitFileAnswer>,
    pub base_commit: Option<ShaAnswer>,
    pub merge_base_commit: Option<ShaAnswer>,
}

impl From<CommitComparisonAnswer> for GithubCommitComparison {
    /// Two commits compared: how far apart they are, and the commits and files between them, in our terms.
    fn from(answer: CommitComparisonAnswer) -> Self {
        Self {
            status: answer.status,
            ahead_by: answer.ahead_by,
            behind_by: answer.behind_by,
            total_commits: answer.total_commits,
            url: answer.url,
            commits: answer.commits.into_iter().map(GithubCommit::from).collect(),
            files: answer
                .files
                .into_iter()
                .map(GithubCommitFile::from)
                .collect(),
            base_commit_sha: answer.base_commit.map(|commit| commit.sha),
            merge_base_commit_sha: answer.merge_base_commit.map(|commit| commit.sha),
        }
    }
}
