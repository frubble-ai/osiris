//! A tag.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A tag.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubTag {
    pub name: String,
    pub commit_sha: String,
    pub zipball_url: String,
    pub tarball_url: String,
}
