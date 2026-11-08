//! The issues and pull requests a search found, as GitHub answers them.

use super::search::SearchIssueAnswer;
use crate::github::types::{GithubSearchIssue, GithubSearchIssueResults};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchIssueResultsAnswer {
    #[serde(default)]
    pub total_count: i64,
    #[serde(default)]
    pub incomplete_results: bool,
    #[serde(default)]
    pub items: Vec<SearchIssueAnswer>,
}

impl From<SearchIssueResultsAnswer> for GithubSearchIssueResults {
    /// The issues and pull requests a search found, in our terms.
    fn from(answer: SearchIssueResultsAnswer) -> Self {
        Self {
            total_count: answer.total_count,
            incomplete_results: answer.incomplete_results,
            items: answer
                .items
                .into_iter()
                .map(GithubSearchIssue::from)
                .collect(),
        }
    }
}
