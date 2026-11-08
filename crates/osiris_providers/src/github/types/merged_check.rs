//! Whether a pull request has been merged.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Whether a pull request has been merged.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubMergedCheck {
    pub merged: bool,
}
