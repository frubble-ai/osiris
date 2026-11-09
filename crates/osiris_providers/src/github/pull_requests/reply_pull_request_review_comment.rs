//! `github.reply_pull_request_review_comment`: a reply to a comment on a pull request's diff.

use crate::{
    ProviderError,
    github::{
        Github, github_id::GithubId, path_segment::path_segment, request::send,
        transformers::review_comment::ReviewCommentAnswer, types::GithubReviewComment,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ReplyPullRequestReviewComment {
    pub owner: String,
    pub repo: String,
    pub pull_number: u64,
    /// The comment replied to. It must be a comment that is not itself a reply.
    pub comment_id: GithubId,
    pub body: String,
}

impl ReplyPullRequestReviewComment {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

/// The reply as GitHub asks for it.
#[derive(Serialize)]
struct Payload<'a> {
    body: &'a str,
}

impl Github {
    /// `POST /repos/{owner}/{repo}/pulls/{pull_number}/comments/{comment_id}/replies`.
    pub async fn reply_pull_request_review_comment(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ReplyPullRequestReviewComment,
    ) -> Result<GithubReviewComment, ProviderError> {
        let path = format!(
            "/repos/{}/{}/pulls/{}/comments/{}/replies",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.pull_number.to_string()),
            path_segment(&input.comment_id.to_string())
        );
        let payload = Payload { body: &input.body };
        let answer: ReviewCommentAnswer =
            send(self, http, token, Method::POST, &path, &[], Some(&payload)).await?;
        Ok(answer.into())
    }
}
