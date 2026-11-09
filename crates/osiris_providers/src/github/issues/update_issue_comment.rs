//! `github.update_issue_comment`: a comment on an issue, its text replaced.

use crate::{
    ProviderError,
    github::{
        Github, github_id::GithubId, path_segment::path_segment, request::send,
        transformers::issue_comment::IssueCommentAnswer, types::GithubIssueComment,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct UpdateIssueComment {
    pub owner: String,
    pub repo: String,
    pub comment_id: GithubId,
    /// The comment's new text, in Markdown.
    pub body: String,
}

impl UpdateIssueComment {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

#[derive(Serialize)]
struct UpdateIssueCommentBody<'a> {
    body: &'a str,
}

impl Github {
    /// `PATCH /repos/{owner}/{repo}/issues/comments/{comment_id}`.
    pub async fn update_issue_comment(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &UpdateIssueComment,
    ) -> Result<GithubIssueComment, ProviderError> {
        let path = format!(
            "/repos/{}/{}/issues/comments/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.comment_id.to_string())
        );
        let body = UpdateIssueCommentBody { body: &input.body };
        let answer: IssueCommentAnswer =
            send(self, http, token, Method::PATCH, &path, &[], Some(&body)).await?;
        Ok(answer.into())
    }
}
