//! Release notes GitHub wrote for a tag, as GitHub answers it.

use super::{reactions::ReactionsAnswer, release_asset::ReleaseAssetAnswer, user::UserAnswer};
use crate::github::types::{
    GithubReactions, GithubRelease, GithubReleaseAsset, GithubReleaseNotes, GithubUser,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ReleaseAnswer {
    pub id: u64,
    #[serde(default)]
    pub tag_name: String,
    pub name: Option<String>,
    pub body: Option<String>,
    #[serde(default)]
    pub draft: bool,
    #[serde(default)]
    pub prerelease: bool,
    #[serde(rename = "html_url", default)]
    pub url: String,
    #[serde(default)]
    pub assets_url: String,
    #[serde(default)]
    pub upload_url: String,
    #[serde(default)]
    pub assets: Vec<ReleaseAssetAnswer>,
    pub tarball_url: Option<String>,
    pub zipball_url: Option<String>,
    #[serde(default)]
    pub target_commitish: String,
    pub author: Option<UserAnswer>,
    #[serde(default)]
    pub created_at: String,
    pub published_at: Option<String>,
    pub reactions: Option<ReactionsAnswer>,
    pub updated_at: Option<String>,
    #[serde(default)]
    pub immutable: bool,
}

impl From<ReleaseAnswer> for GithubRelease {
    /// A release, in our terms.
    fn from(answer: ReleaseAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            tag_name: answer.tag_name,
            name: answer.name,
            body: answer.body,
            draft: answer.draft,
            prerelease: answer.prerelease,
            url: answer.url,
            assets_url: answer.assets_url,
            upload_url: answer.upload_url,
            assets: answer
                .assets
                .into_iter()
                .map(GithubReleaseAsset::from)
                .collect(),
            tarball_url: answer.tarball_url,
            zipball_url: answer.zipball_url,
            target_commitish: answer.target_commitish,
            author: answer.author.map(GithubUser::from),
            created_at: answer.created_at,
            published_at: answer.published_at,
            reactions: answer.reactions.map(GithubReactions::from),
            updated_at: answer.updated_at,
            immutable: answer.immutable,
        }
    }
}

#[derive(Deserialize)]
pub struct ReleaseNotesAnswer {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub body: String,
}

impl From<ReleaseNotesAnswer> for GithubReleaseNotes {
    /// Release notes GitHub wrote for a tag, in our terms.
    fn from(answer: ReleaseNotesAnswer) -> Self {
        Self {
            name: answer.name,
            body: answer.body,
        }
    }
}
