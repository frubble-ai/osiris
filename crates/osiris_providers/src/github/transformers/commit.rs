//! A commit, as GitHub answers it.

use super::user::UserAnswer;
use crate::github::types::{GithubCommit, GithubCommitFile, GithubCommitStats, GithubUser};
use serde::Deserialize;

/// A parent commit, as GitHub names it.
#[derive(Deserialize)]
pub struct ShaAnswer {
    pub sha: String,
}

#[derive(Deserialize)]
pub struct CommitStatsAnswer {
    #[serde(default)]
    pub additions: i64,
    #[serde(default)]
    pub deletions: i64,
    #[serde(default)]
    pub total: i64,
}

impl From<CommitStatsAnswer> for GithubCommitStats {
    /// How much a commit changed, in our terms.
    fn from(answer: CommitStatsAnswer) -> Self {
        Self {
            additions: answer.additions,
            deletions: answer.deletions,
            total: answer.total,
        }
    }
}

#[derive(Deserialize)]
pub struct CommitFileAnswer {
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
}

impl From<CommitFileAnswer> for GithubCommitFile {
    /// A file a commit changed, in our terms.
    fn from(answer: CommitFileAnswer) -> Self {
        Self {
            filename: answer.filename,
            status: answer.status,
            additions: answer.additions,
            deletions: answer.deletions,
            changes: answer.changes,
            patch: answer.patch,
        }
    }
}

#[derive(Deserialize)]
pub struct CommitAnswer {
    #[serde(default)]
    pub sha: String,
    #[serde(rename = "html_url", default)]
    pub url: String,
    pub author: Option<UserAnswer>,
    pub committer: Option<UserAnswer>,
    #[serde(default)]
    pub parents: Vec<ShaAnswer>,
    pub stats: Option<CommitStatsAnswer>,
    /// Absent in a list of commits, which GitHub answers without them.
    #[serde(default)]
    pub files: Option<Vec<CommitFileAnswer>>,
    #[serde(default)]
    pub commit: Option<CommitCommitAnswer>,
}

#[derive(Deserialize, Default)]
pub struct CommitCommitAnswer {
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub author: Option<CommitCommitAuthorAnswer>,
    #[serde(default)]
    pub committer: Option<CommitCommitCommitterAnswer>,
}

#[derive(Deserialize, Default)]
pub struct CommitCommitAuthorAnswer {
    #[serde(rename = "name", default)]
    pub author_name: Option<String>,
    #[serde(rename = "email", default)]
    pub author_email: Option<String>,
    #[serde(rename = "date", default)]
    pub authored_at: Option<String>,
}

#[derive(Deserialize, Default)]
pub struct CommitCommitCommitterAnswer {
    #[serde(rename = "name", default)]
    pub committer_name: Option<String>,
    #[serde(rename = "date", default)]
    pub committed_at: Option<String>,
}

impl From<CommitAnswer> for GithubCommit {
    /// A commit, in our terms.
    fn from(answer: CommitAnswer) -> Self {
        let commit = answer.commit.unwrap_or_default();
        Self {
            sha: answer.sha,
            url: answer.url,
            author: answer.author.map(GithubUser::from),
            committer: answer.committer.map(GithubUser::from),
            parents: answer
                .parents
                .into_iter()
                .map(|parent| parent.sha)
                .collect(),
            stats: answer.stats.map(GithubCommitStats::from),
            files: answer
                .files
                .map(|files| files.into_iter().map(GithubCommitFile::from).collect()),
            message: commit.message,
            author_name: commit
                .author
                .as_ref()
                .and_then(|part| part.author_name.clone()),
            author_email: commit
                .author
                .as_ref()
                .and_then(|part| part.author_email.clone()),
            authored_at: commit
                .author
                .as_ref()
                .and_then(|part| part.authored_at.clone()),
            committer_name: commit
                .committer
                .as_ref()
                .and_then(|part| part.committer_name.clone()),
            committed_at: commit
                .committer
                .as_ref()
                .and_then(|part| part.committed_at.clone()),
        }
    }
}
