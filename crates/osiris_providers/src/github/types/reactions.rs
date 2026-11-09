//! How many of each reaction an issue or a comment has.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// How many of each reaction an issue or a comment has: GitHub's `+1` is `plus_one`, its `-1`
/// `minus_one`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct GithubReactions {
    pub total_count: i64,
    pub plus_one: i64,
    pub minus_one: i64,
    pub laugh: i64,
    pub confused: i64,
    pub heart: i64,
    pub hooray: i64,
    pub rocket: i64,
    pub eyes: i64,
}
