//! A git reference (a branch or a tag) and the object it points at.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A git reference (a branch or a tag) and the object it points at.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubGitRef {
    pub ref_name: String,
    pub sha: String,
    pub object_type: String,
}
