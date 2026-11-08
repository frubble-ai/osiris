//! A fork's branch brought up to date with its upstream.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A fork's branch brought up to date with its upstream.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct GithubForkSync {
    /// What GitHub did, in its own words.
    pub message: String,
    /// `merge`, `fast-forward` or `none` today, as GitHub names it. An open string.
    pub merge_type: Option<String>,
    /// The upstream branch, as `owner:branch`.
    pub base_branch: Option<String>,
}
