//! A check run on a commit, as GitHub answers it.

use crate::github::types::{GithubCheckRun, GithubCheckRunOutput};
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
    pub app: Option<CheckRunAppAnswer>,
    pub check_suite: Option<CheckRunSuiteAnswer>,
    pub external_id: Option<String>,
    pub output: Option<CheckRunOutputAnswer>,
}

/// The app that made a check run, of which we keep the slug.
#[derive(Deserialize)]
pub struct CheckRunAppAnswer {
    pub slug: Option<String>,
}

/// The suite a check run is part of, of which we keep the id.
#[derive(Deserialize)]
pub struct CheckRunSuiteAnswer {
    pub id: u64,
}

#[derive(Deserialize)]
pub struct CheckRunOutputAnswer {
    pub title: Option<String>,
    pub summary: Option<String>,
    pub text: Option<String>,
    #[serde(default)]
    pub annotations_count: i64,
}

impl From<CheckRunOutputAnswer> for GithubCheckRunOutput {
    /// What a check run reported, in our terms.
    fn from(answer: CheckRunOutputAnswer) -> Self {
        Self {
            title: answer.title,
            summary: answer.summary,
            text: answer.text,
            annotations_count: answer.annotations_count,
        }
    }
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
            app: answer.app.and_then(|app| app.slug),
            check_suite_id: answer.check_suite.map(|suite| suite.id.to_string()),
            external_id: answer.external_id,
            output: answer.output.map(GithubCheckRunOutput::from),
        }
    }
}
