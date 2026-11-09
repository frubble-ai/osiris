//! A git reference (a branch or a tag) and the object it points at, as GitHub answers it.

use crate::github::types::GithubGitRef;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GitRefAnswer {
    #[serde(rename = "ref", default)]
    pub ref_name: String,
    #[serde(default)]
    pub object: Option<GitRefObjectAnswer>,
}

#[derive(Deserialize, Default)]
pub struct GitRefObjectAnswer {
    #[serde(default)]
    pub sha: String,
    #[serde(rename = "type", default)]
    pub object_type: String,
}

impl From<GitRefAnswer> for GithubGitRef {
    /// A git reference (a branch or a tag) and the object it points at, in our terms.
    fn from(answer: GitRefAnswer) -> Self {
        let object = answer.object.unwrap_or_default();
        Self {
            ref_name: answer.ref_name,
            sha: object.sha,
            object_type: object.object_type,
        }
    }
}
