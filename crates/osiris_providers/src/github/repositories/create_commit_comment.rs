//! `github.create_commit_comment`: a comment on a commit, on the whole commit or on one line of
//! its diff.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send,
        transformers::commit_comment::CommitCommentAnswer, trimmed::trimmed,
        types::GithubCommitComment,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateCommitComment {
    pub owner: String,
    pub repo: String,
    pub commit_sha: String,
    pub body: String,
    /// The file to comment on, relative to the repository's root.
    pub path: Option<String>,
    /// The line of the file's diff to comment on, from 1.
    #[schema(minimum = 1)]
    pub position: Option<u32>,
}

impl CreateCommitComment {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

#[derive(Serialize)]
struct Comment<'a> {
    body: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<u32>,
}

impl Github {
    /// `POST /repos/{owner}/{repo}/commits/{commit_sha}/comments`.
    pub async fn create_commit_comment(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &CreateCommitComment,
    ) -> Result<GithubCommitComment, ProviderError> {
        let path = format!(
            "/repos/{}/{}/commits/{}/comments",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.commit_sha)
        );
        let body = Comment {
            body: &input.body,
            path: trimmed(input.path.as_deref()),
            position: input.position,
        };
        let answer: CommitCommentAnswer =
            send(self, http, token, Method::POST, &path, &[], Some(&body)).await?;
        Ok(answer.into())
    }
}
