//! `github.update_pull_request_review_comment`: a comment on a pull request's diff, its text
//! replaced.

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
pub struct UpdatePullRequestReviewComment {
    pub owner: String,
    pub repo: String,
    pub comment_id: u64,
    /// The comment's new text.
    pub body: String,
}

impl UpdatePullRequestReviewComment {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

/// The change as GitHub asks for it.
#[derive(Serialize)]
struct Payload<'a> {
    body: &'a str,
}

impl Github {
    /// `PATCH /repos/{owner}/{repo}/pulls/comments/{comment_id}`.
    pub async fn update_pull_request_review_comment(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &UpdatePullRequestReviewComment,
    ) -> Result<GithubReviewComment, ProviderError> {
        let path = format!(
            "/repos/{}/{}/pulls/comments/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.comment_id.to_string())
        );
        let payload = Payload { body: &input.body };
        let answer: ReviewCommentAnswer =
            send(self, http, token, Method::PATCH, &path, &[], Some(&payload)).await?;
        Ok(answer.into())
    }
}
