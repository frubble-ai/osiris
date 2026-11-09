//! Release notes GitHub wrote for a tag, and its parts.

use super::{GithubReactions, GithubReleaseAsset, GithubUser};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A release.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubRelease {
    pub id: String,
    pub tag_name: String,
    pub name: Option<String>,
    pub body: Option<String>,
    pub draft: bool,
    pub prerelease: bool,
    pub url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub assets: Vec<GithubReleaseAsset>,
    pub tarball_url: Option<String>,
    pub zipball_url: Option<String>,
    pub target_commitish: String,
    pub author: Option<GithubUser>,
    pub created_at: String,
    pub published_at: Option<String>,
    pub reactions: Option<GithubReactions>,
}

/// Release notes GitHub wrote for a tag.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubReleaseNotes {
    pub name: String,
    pub body: String,
}
