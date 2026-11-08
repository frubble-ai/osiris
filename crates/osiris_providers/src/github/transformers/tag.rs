//! A tag, as GitHub answers it.

use crate::github::types::GithubTag;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TagAnswer {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub zipball_url: String,
    #[serde(default)]
    pub tarball_url: String,
    #[serde(default)]
    pub commit: Option<TagCommitAnswer>,
}

#[derive(Deserialize, Default)]
pub struct TagCommitAnswer {
    #[serde(rename = "sha", default)]
    pub commit_sha: String,
}

impl From<TagAnswer> for GithubTag {
    /// A tag, in our terms.
    fn from(answer: TagAnswer) -> Self {
        let commit = answer.commit.unwrap_or_default();
        Self {
            name: answer.name,
            zipball_url: answer.zipball_url,
            tarball_url: answer.tarball_url,
            commit_sha: commit.commit_sha,
        }
    }
}
