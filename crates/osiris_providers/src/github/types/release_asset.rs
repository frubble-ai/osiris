//! A file attached to a release.

use super::GithubUser;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A file attached to a release.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubReleaseAsset {
    pub id: String,
    pub name: String,
    pub label: Option<String>,
    pub state: String,
    pub content_type: String,
    pub size: i64,
    pub download_count: i64,
    pub download_url: String,
    pub uploader: Option<GithubUser>,
    pub created_at: String,
    pub updated_at: String,
}
