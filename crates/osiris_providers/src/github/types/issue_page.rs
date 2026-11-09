//! A page of a repository's issues, with the pull requests GitHub mixes in left out.

use super::GithubIssue;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A page of a repository's issues, with the pull requests GitHub mixes in left out.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubIssuePage {
    /// The page's issues, without its pull requests.
    pub issues: Vec<GithubIssue>,
    /// How many items GitHub answered on this page before the pull requests were left out. A
    /// caller reading page after page goes on while this equals the page size asked for (30 when
    /// none was asked), even when `issues` comes back short or empty.
    pub fetched: u64,
}
