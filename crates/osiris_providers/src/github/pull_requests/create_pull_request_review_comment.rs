//! `github.create_pull_request_review_comment`: a comment on a line, or a range of lines, of a pull
//! request's diff.

use super::DiffSide;
use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send,
        transformers::review_comment::ReviewCommentAnswer, types::GithubReviewComment,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreatePullRequestReviewComment {
    pub owner: String,
    pub repo: String,
    pub pull_number: u64,
    pub body: String,
    /// The commit the comment is on, usually the pull request's latest.
    pub commit_id: String,
    /// The file's path in the repository.
    pub path: String,
    /// The line the comment is on, or the last line of a range of lines.
    pub line: Option<u64>,
    /// The side of the diff `line` is on.
    pub side: Option<DiffSide>,
    /// The first line of a range of lines the comment is on.
    pub start_line: Option<u64>,
    /// The side of the diff `start_line` is on.
    pub start_side: Option<DiffSide>,
}

impl CreatePullRequestReviewComment {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

/// The comment as GitHub asks for it.
#[derive(Serialize)]
struct Payload<'a> {
    body: &'a str,
    commit_id: &'a str,
    path: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    side: Option<DiffSide>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_line: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_side: Option<DiffSide>,
}

impl Github {
    /// `POST /repos/{owner}/{repo}/pulls/{pull_number}/comments`.
    pub async fn create_pull_request_review_comment(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &CreatePullRequestReviewComment,
    ) -> Result<GithubReviewComment, ProviderError> {
        let path = format!(
            "/repos/{}/{}/pulls/{}/comments",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.pull_number.to_string())
        );
        let payload = Payload {
            body: &input.body,
            commit_id: &input.commit_id,
            path: &input.path,
            line: input.line,
            side: input.side,
            start_line: input.start_line,
            start_side: input.start_side,
        };
        let answer: ReviewCommentAnswer =
            send(self, http, token, Method::POST, &path, &[], Some(&payload)).await?;
        Ok(answer.into())
    }
}
