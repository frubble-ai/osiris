//! What a search answers, as GitHub answers it: `total_count`, `incomplete_results` and the page
//! of `items`.

use super::search::{
    SearchCodeAnswer, SearchCommitAnswer, SearchLabelAnswer, SearchRepositoryAnswer,
    SearchTopicAnswer, SearchUserAnswer,
};
use crate::github::types::{
    GithubCodeSearchResults, GithubCommitSearchResults, GithubLabelSearchResults,
    GithubRepositorySearchResults, GithubSearchCode, GithubSearchCommit, GithubSearchLabel,
    GithubSearchRepository, GithubSearchTopic, GithubSearchUser, GithubTopicSearchResults,
    GithubUserSearchResults,
};
use serde::Deserialize;

/// A page of any search, its items still as GitHub sends them.
#[derive(Deserialize)]
pub struct SearchResultsAnswer<T> {
    #[serde(default)]
    pub total_count: i64,
    #[serde(default)]
    pub incomplete_results: bool,
    #[serde(default = "Vec::new")]
    pub items: Vec<T>,
}

impl From<SearchResultsAnswer<SearchRepositoryAnswer>> for GithubRepositorySearchResults {
    /// The repositories a search found, in our terms.
    fn from(answer: SearchResultsAnswer<SearchRepositoryAnswer>) -> Self {
        Self {
            total_count: answer.total_count,
            incomplete_results: answer.incomplete_results,
            repositories: answer
                .items
                .into_iter()
                .map(GithubSearchRepository::from)
                .collect(),
        }
    }
}

impl From<SearchResultsAnswer<SearchUserAnswer>> for GithubUserSearchResults {
    /// The users a search found, in our terms.
    fn from(answer: SearchResultsAnswer<SearchUserAnswer>) -> Self {
        Self {
            total_count: answer.total_count,
            incomplete_results: answer.incomplete_results,
            items: answer
                .items
                .into_iter()
                .map(GithubSearchUser::from)
                .collect(),
        }
    }
}

impl From<SearchResultsAnswer<SearchCommitAnswer>> for GithubCommitSearchResults {
    /// The commits a search found, in our terms.
    fn from(answer: SearchResultsAnswer<SearchCommitAnswer>) -> Self {
        Self {
            total_count: answer.total_count,
            incomplete_results: answer.incomplete_results,
            items: answer
                .items
                .into_iter()
                .map(GithubSearchCommit::from)
                .collect(),
        }
    }
}

impl From<SearchResultsAnswer<SearchCodeAnswer>> for GithubCodeSearchResults {
    /// The files a code search found, in our terms.
    fn from(answer: SearchResultsAnswer<SearchCodeAnswer>) -> Self {
        Self {
            total_count: answer.total_count,
            incomplete_results: answer.incomplete_results,
            items: answer
                .items
                .into_iter()
                .map(GithubSearchCode::from)
                .collect(),
        }
    }
}

impl From<SearchResultsAnswer<SearchLabelAnswer>> for GithubLabelSearchResults {
    /// The labels a search found, in our terms.
    fn from(answer: SearchResultsAnswer<SearchLabelAnswer>) -> Self {
        Self {
            total_count: answer.total_count,
            incomplete_results: answer.incomplete_results,
            items: answer
                .items
                .into_iter()
                .map(GithubSearchLabel::from)
                .collect(),
        }
    }
}

impl From<SearchResultsAnswer<SearchTopicAnswer>> for GithubTopicSearchResults {
    /// The topics a search found, in our terms.
    fn from(answer: SearchResultsAnswer<SearchTopicAnswer>) -> Self {
        Self {
            total_count: answer.total_count,
            incomplete_results: answer.incomplete_results,
            items: answer
                .items
                .into_iter()
                .map(GithubSearchTopic::from)
                .collect(),
        }
    }
}
