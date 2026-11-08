//! `github.delete_file`: a file of a repository deleted, in a commit of its own. Deleting under
//! `.github/workflows` may need GitHub's `workflow` scope too.

use crate::{
    ProviderError,
    github::{
        Github, repositories::contents_path::contents_path, request::send,
        transformers::content::ContentChangeAnswer, trimmed::trimmed, types::GithubContentChange,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct DeleteFile {
    pub owner: String,
    pub repo: String,
    /// The file, such as `docs/guide.md`.
    pub path: String,
    /// The commit's message.
    pub message: String,
    /// The SHA of the file being deleted.
    pub sha: String,
    /// The branch to commit to; the default branch when left out.
    pub branch: Option<String>,
}

impl DeleteFile {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

#[derive(Serialize)]
struct Body<'a> {
    message: &'a str,
    sha: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    branch: Option<&'a str>,
}

impl Github {
    /// `DELETE /repos/{owner}/{repo}/contents/{path}`.
    pub async fn delete_file(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &DeleteFile,
    ) -> Result<GithubContentChange, ProviderError> {
        let path = contents_path(&input.owner, &input.repo, Some(&input.path));
        let body = Body {
            message: &input.message,
            sha: &input.sha,
            branch: trimmed(input.branch.as_deref()),
        };
        let answer: ContentChangeAnswer =
            send(self, http, token, Method::DELETE, &path, &[], Some(&body)).await?;
        Ok(answer.into())
    }
}
