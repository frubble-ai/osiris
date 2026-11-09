//! `github.create_issue_comment`: a new comment on an issue.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send,
        transformers::issue_comment::IssueCommentAnswer, types::GithubIssueComment,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateIssueComment {
    pub owner: String,
    pub repo: String,
    pub issue_number: u64,
    /// The comment's text, in Markdown.
    pub body: String,
}

impl CreateIssueComment {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

#[derive(Serialize)]
struct CreateIssueCommentBody<'a> {
    body: &'a str,
}

impl Github {
    /// `POST /repos/{owner}/{repo}/issues/{issue_number}/comments`.
    pub async fn create_issue_comment(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &CreateIssueComment,
    ) -> Result<GithubIssueComment, ProviderError> {
        let path = format!(
            "/repos/{}/{}/issues/{}/comments",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.issue_number.to_string())
        );
        let body = CreateIssueCommentBody { body: &input.body };
        let answer: IssueCommentAnswer =
            send(self, http, token, Method::POST, &path, &[], Some(&body)).await?;
        Ok(answer.into())
    }
}
