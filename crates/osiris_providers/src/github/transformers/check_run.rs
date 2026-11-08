//! A check run on a commit, as GitHub answers it.

use crate::github::types::GithubCheckRun;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CheckRunAnswer {
    pub id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub status: String,
    pub conclusion: Option<String>,
    #[serde(default)]
    pub head_sha: String,
    #[serde(rename = "html_url")]
    pub url: Option<String>,
    pub details_url: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
}

impl From<CheckRunAnswer> for GithubCheckRun {
    /// A check run on a commit, in our terms.
    fn from(answer: CheckRunAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            name: answer.name,
            status: answer.status,
            conclusion: answer.conclusion,
            head_sha: answer.head_sha,
            url: answer.url,
            details_url: answer.details_url,
            started_at: answer.started_at,
            completed_at: answer.completed_at,
        }
    }
}
