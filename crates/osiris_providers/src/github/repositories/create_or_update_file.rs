//! `github.create_or_update_file`: a file of a repository written, in a commit of its own.
//! Writing under `.github/workflows` may need GitHub's `workflow` scope too.

use crate::{
    ProviderError,
    github::{
        Github, repositories::contents_path::contents_path, request::send,
        transformers::content::ContentChangeAnswer, trimmed::trimmed, types::GithubContentChange,
    },
};
use base64::{Engine, engine::general_purpose::STANDARD};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateOrUpdateFile {
    pub owner: String,
    pub repo: String,
    /// The file, such as `docs/guide.md`.
    pub path: String,
    /// The commit's message.
    pub message: String,
    /// The file's new content as text. Ignored when `content_base64` is given; an empty file when
    /// both are left out.
    pub content: Option<String>,
    /// The file's new content, base64-encoded, for a binary file.
    pub content_base64: Option<String>,
    /// The SHA of the file being replaced; required to update an existing file.
    pub sha: Option<String>,
    /// The branch to commit to; the default branch when left out.
    pub branch: Option<String>,
}

impl CreateOrUpdateFile {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];

    /// The content GitHub is sent, base64-encoded: `content_base64` without its line breaks when
    /// it is given, `content` encoded otherwise.
    fn encoded_content(&self) -> String {
        match self.content_base64.as_deref() {
            Some(encoded) if !encoded.is_empty() => encoded.replace(['\r', '\n'], ""),
            _ => STANDARD.encode(self.content.as_deref().unwrap_or_default()),
        }
    }
}

#[derive(Serialize)]
struct Body<'a> {
    message: &'a str,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    sha: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<&'a str>,
}

impl Github {
    /// `PUT /repos/{owner}/{repo}/contents/{path}`.
    pub async fn create_or_update_file(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &CreateOrUpdateFile,
    ) -> Result<GithubContentChange, ProviderError> {
        let path = contents_path(&input.owner, &input.repo, Some(&input.path));
        let body = Body {
            message: &input.message,
            content: input.encoded_content(),
            sha: trimmed(input.sha.as_deref()),
            branch: trimmed(input.branch.as_deref()),
        };
        let answer: ContentChangeAnswer =
            send(self, http, token, Method::PUT, &path, &[], Some(&body)).await?;
        Ok(answer.into())
    }
}
