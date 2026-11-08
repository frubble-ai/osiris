//! A file attached to a release, as GitHub answers it.

use super::user::UserAnswer;
use crate::github::types::{GithubReleaseAsset, GithubUser};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ReleaseAssetAnswer {
    pub id: u64,
    #[serde(default)]
    pub name: String,
    pub label: Option<String>,
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub content_type: String,
    #[serde(default)]
    pub size: i64,
    #[serde(default)]
    pub download_count: i64,
    #[serde(rename = "browser_download_url", default)]
    pub download_url: String,
    pub uploader: Option<UserAnswer>,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

impl From<ReleaseAssetAnswer> for GithubReleaseAsset {
    /// A file attached to a release, in our terms.
    fn from(answer: ReleaseAssetAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            name: answer.name,
            label: answer.label,
            state: answer.state,
            content_type: answer.content_type,
            size: answer.size,
            download_count: answer.download_count,
            download_url: answer.download_url,
            uploader: answer.uploader.map(GithubUser::from),
            created_at: answer.created_at,
            updated_at: answer.updated_at,
        }
    }
}
