//! What a search answers: how many matches GitHub counted, whether it gave up before counting them
//! all, and the page of matches asked for, one type per kind of search.

use super::{
    GithubSearchCode, GithubSearchCommit, GithubSearchLabel, GithubSearchRepository,
    GithubSearchTopic, GithubSearchUser,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// The repositories a search found.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubRepositorySearchResults {
    /// Every match GitHub counted, across all pages.
    pub total_count: i64,
    /// GitHub stopped searching before it had looked at everything, so matches may be missing.
    pub incomplete_results: bool,
    pub repositories: Vec<GithubSearchRepository>,
}

/// The users a search found.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubUserSearchResults {
    /// Every match GitHub counted, across all pages.
    pub total_count: i64,
    /// GitHub stopped searching before it had looked at everything, so matches may be missing.
    pub incomplete_results: bool,
    pub items: Vec<GithubSearchUser>,
}

/// The commits a search found.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubCommitSearchResults {
    /// Every match GitHub counted, across all pages.
    pub total_count: i64,
    /// GitHub stopped searching before it had looked at everything, so matches may be missing.
    pub incomplete_results: bool,
    pub items: Vec<GithubSearchCommit>,
}

/// The files a code search found.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubCodeSearchResults {
    /// Every match GitHub counted, across all pages.
    pub total_count: i64,
    /// GitHub stopped searching before it had looked at everything, so matches may be missing.
    pub incomplete_results: bool,
    pub items: Vec<GithubSearchCode>,
}

/// The labels a search found.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubLabelSearchResults {
    /// Every match GitHub counted, across all pages.
    pub total_count: i64,
    /// GitHub stopped searching before it had looked at everything, so matches may be missing.
    pub incomplete_results: bool,
    pub items: Vec<GithubSearchLabel>,
}

/// The topics a search found.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubTopicSearchResults {
    /// Every match GitHub counted, across all pages.
    pub total_count: i64,
    /// GitHub stopped searching before it had looked at everything, so matches may be missing.
    pub incomplete_results: bool,
    pub items: Vec<GithubSearchTopic>,
}
