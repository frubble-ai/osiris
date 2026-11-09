//! A file a pull request changes, as GitHub answers it.

use crate::github::types::GithubPullRequestFile;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PullRequestFileAnswer {
    #[serde(default)]
    pub sha: String,
    #[serde(default)]
    pub filename: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub additions: i64,
    #[serde(default)]
    pub deletions: i64,
    #[serde(default)]
    pub changes: i64,
    pub patch: Option<String>,
    pub previous_filename: Option<String>,
    pub blob_url: Option<String>,
}

impl From<PullRequestFileAnswer> for GithubPullRequestFile {
    /// A file a pull request changes, in our terms.
    fn from(answer: PullRequestFileAnswer) -> Self {
        Self {
            sha: answer.sha,
            filename: answer.filename,
            status: answer.status,
            additions: answer.additions,
            deletions: answer.deletions,
            changes: answer.changes,
            patch: answer.patch,
            previous_filename: answer.previous_filename,
            blob_url: answer.blob_url,
        }
    }
}
