//! A branch.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A branch: its name, the commit it points at, and whether it is protected.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct GithubBranch {
    pub name: String,
    pub commit_sha: String,
    pub protected: bool,
}
