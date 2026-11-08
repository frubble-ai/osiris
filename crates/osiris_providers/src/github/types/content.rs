//! A file written or deleted, and the commit that did it, and its parts.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A file, a directory, a symlink or a submodule of a repository's contents.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubContentEntry {
    pub entry_type: String,
    pub name: String,
    pub path: String,
    pub sha: String,
    pub size: i64,
    pub url: Option<String>,
    pub download_url: Option<String>,
}

/// A file and its content.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubFileContent {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub size: i64,
    pub url: Option<String>,
    pub download_url: Option<String>,
    pub encoding: String,
    /// The content, base64-encoded, without the line breaks GitHub wraps it in.
    pub content_base64: String,
    /// The content as text, when it is base64-encoded UTF-8; absent for a binary file.
    pub decoded_content: Option<String>,
}

/// The commit a contents change made.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubContentCommit {
    pub sha: String,
    pub url: String,
    pub message: String,
}

/// A file written or deleted, and the commit that did it.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct GithubContentChange {
    pub content: Option<GithubContentEntry>,
    pub commit: GithubContentCommit,
}
