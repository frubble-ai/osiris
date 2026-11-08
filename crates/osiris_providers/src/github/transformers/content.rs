//! A file written or deleted, and the commit that did it, as GitHub answers it.

use crate::github::types::{
    GithubContentChange, GithubContentCommit, GithubContentEntry, GithubFileContent,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ContentEntryAnswer {
    #[serde(rename = "type", default)]
    pub entry_type: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub sha: String,
    #[serde(default)]
    pub size: i64,
    #[serde(rename = "html_url")]
    pub url: Option<String>,
    pub download_url: Option<String>,
}

impl From<ContentEntryAnswer> for GithubContentEntry {
    /// A file, a directory, a symlink or a submodule of a repository's contents, in our terms.
    fn from(answer: ContentEntryAnswer) -> Self {
        Self {
            entry_type: answer.entry_type,
            name: answer.name,
            path: answer.path,
            sha: answer.sha,
            size: answer.size,
            url: answer.url,
            download_url: answer.download_url,
        }
    }
}

#[derive(Deserialize)]
pub struct FileContentAnswer {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub sha: String,
    #[serde(default)]
    pub size: i64,
    #[serde(rename = "html_url")]
    pub url: Option<String>,
    pub download_url: Option<String>,
    #[serde(default)]
    pub encoding: String,
    #[serde(rename = "content", default)]
    pub content_base64: String,
}

impl From<FileContentAnswer> for GithubFileContent {
    /// A file and its content, in our terms.
    fn from(answer: FileContentAnswer) -> Self {
        Self {
            name: answer.name,
            path: answer.path,
            sha: answer.sha,
            size: answer.size,
            url: answer.url,
            download_url: answer.download_url,
            encoding: answer.encoding,
            content_base64: answer.content_base64,
        }
    }
}

#[derive(Deserialize)]
pub struct ContentCommitAnswer {
    #[serde(default)]
    pub sha: String,
    #[serde(rename = "html_url", default)]
    pub url: String,
    #[serde(default)]
    pub message: String,
}

impl From<ContentCommitAnswer> for GithubContentCommit {
    /// The commit a contents change made, in our terms.
    fn from(answer: ContentCommitAnswer) -> Self {
        Self {
            sha: answer.sha,
            url: answer.url,
            message: answer.message,
        }
    }
}

#[derive(Deserialize)]
pub struct ContentChangeAnswer {
    pub content: Option<ContentEntryAnswer>,
    pub commit: ContentCommitAnswer,
}

impl From<ContentChangeAnswer> for GithubContentChange {
    /// A file written or deleted, and the commit that did it, in our terms.
    fn from(answer: ContentChangeAnswer) -> Self {
        Self {
            content: answer.content.map(GithubContentEntry::from),
            commit: GithubContentCommit::from(answer.commit),
        }
    }
}
