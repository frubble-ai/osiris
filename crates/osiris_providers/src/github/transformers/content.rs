//! A file written or deleted, and the commit that did it, as GitHub answers it.

use crate::github::types::{
    GithubContentChange, GithubContentCommit, GithubContentEntry, GithubFileContent,
};
use base64::{Engine, engine::general_purpose::STANDARD};
use serde::Deserialize;

/// `GET /repos/{owner}/{repo}/contents/{path}`: a directory's entries, or a single item.
#[derive(Deserialize)]
#[serde(untagged)]
pub enum ContentsAnswer {
    Directory(Vec<ContentEntryAnswer>),
    Item(Box<FileContentAnswer>),
}

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
    /// `file`, `dir`, `symlink` or `submodule`.
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
    #[serde(default)]
    pub encoding: String,
    #[serde(rename = "content", default)]
    pub content_base64: String,
}

/// The text a file's base64 `content` holds, as `OpenConnector` decodes it: empty for an empty
/// file, and absent when the encoding is not base64 or the bytes are not UTF-8.
fn decode_content(content_base64: &str, encoding: &str) -> Option<String> {
    if content_base64.is_empty() {
        return Some(String::new());
    }
    if !encoding.is_empty() && encoding != "base64" {
        return None;
    }
    let bytes = STANDARD.decode(content_base64).ok()?;
    String::from_utf8(bytes).ok()
}

impl From<FileContentAnswer> for GithubFileContent {
    /// A file and its content, in our terms, the content's line breaks removed and its text
    /// decoded when it is text.
    fn from(answer: FileContentAnswer) -> Self {
        let content_base64 = answer.content_base64.replace('\n', "");
        let decoded_content = decode_content(&content_base64, &answer.encoding);
        Self {
            name: answer.name,
            path: answer.path,
            sha: answer.sha,
            size: answer.size,
            url: answer.url,
            download_url: answer.download_url,
            encoding: answer.encoding,
            content_base64,
            decoded_content,
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
