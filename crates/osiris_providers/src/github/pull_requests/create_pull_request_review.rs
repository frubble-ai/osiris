//! `github.create_pull_request_review`: a review of a pull request, with comments on lines of its
//! diff if given. Without an `event` the review is left pending, to be submitted later.

use super::{ReviewCommentInput, ReviewEvent, review_comment_input::ReviewCommentBody};
use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send, transformers::review::ReviewAnswer,
        types::GithubReview,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreatePullRequestReview {
    pub owner: String,
    pub repo: String,
    pub pull_number: u64,
    pub body: Option<String>,
    /// What the review says of the pull request. Left out, the review stays pending.
    pub event: Option<ReviewEvent>,
    /// The commit the review is of; the pull request's latest commit when left out.
    pub commit_id: Option<String>,
    /// Comments on lines of the diff, made with the review.
    pub comments: Option<Vec<ReviewCommentInput>>,
}

impl CreatePullRequestReview {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

/// The review as GitHub asks for it.
#[derive(Serialize)]
struct Payload<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event: Option<ReviewEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comments: Option<Vec<ReviewCommentBody<'a>>>,
}

impl Github {
    /// `POST /repos/{owner}/{repo}/pulls/{pull_number}/reviews`.
    pub async fn create_pull_request_review(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &CreatePullRequestReview,
    ) -> Result<GithubReview, ProviderError> {
        let path = format!(
            "/repos/{}/{}/pulls/{}/reviews",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.pull_number.to_string())
        );
        let payload = Payload {
            body: input.body.as_deref(),
            event: input.event,
            commit_id: input.commit_id.as_deref(),
            comments: input
                .comments
                .as_ref()
                .map(|comments| comments.iter().map(ReviewCommentBody::from).collect()),
        };
        let answer: ReviewAnswer =
            send(self, http, token, Method::POST, &path, &[], Some(&payload)).await?;
        Ok(answer.into())
    }
}
