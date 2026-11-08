//! The issues and pull requests a search found, and how many there are in all.

use super::GithubSearchIssue;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// The issues and pull requests a search found, and how many there are in all.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubSearchIssueResults {
    /// How many items match, across every page.
    pub total_count: i64,
    /// Whether GitHub stopped searching before it had looked at everything.
    pub incomplete_results: bool,
    pub items: Vec<GithubSearchIssue>,
}
