//! `github.submit_pull_request_review`: a pending review of a pull request, submitted.

use super::ReviewEvent;
use crate::{
    ProviderError,
    github::{
        Github, github_id::GithubId, path_segment::path_segment, request::send,
        transformers::review::ReviewAnswer, types::GithubReview,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct SubmitPullRequestReview {
    pub owner: String,
    pub repo: String,
    pub pull_number: u64,
    pub review_id: GithubId,
    /// What the review says of the pull request.
    pub event: ReviewEvent,
    pub body: Option<String>,
}

impl SubmitPullRequestReview {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

/// The submission as GitHub asks for it.
#[derive(Serialize)]
struct Payload<'a> {
    event: ReviewEvent,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<&'a str>,
}

impl Github {
    /// `POST /repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}/events`.
    pub async fn submit_pull_request_review(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &SubmitPullRequestReview,
    ) -> Result<GithubReview, ProviderError> {
        let path = format!(
            "/repos/{}/{}/pulls/{}/reviews/{}/events",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.pull_number.to_string()),
            path_segment(&input.review_id.to_string())
        );
        let payload = Payload {
            event: input.event,
            body: input.body.as_deref(),
        };
        let answer: ReviewAnswer =
            send(self, http, token, Method::POST, &path, &[], Some(&payload)).await?;
        Ok(answer.into())
    }
}
