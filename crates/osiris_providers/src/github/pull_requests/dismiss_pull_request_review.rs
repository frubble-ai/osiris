//! `github.dismiss_pull_request_review`: a submitted review of a pull request, dismissed.

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
pub struct DismissPullRequestReview {
    pub owner: String,
    pub repo: String,
    pub pull_number: u64,
    pub review_id: u64,
    /// Why the review is dismissed.
    pub message: String,
}

impl DismissPullRequestReview {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

/// The dismissal as GitHub asks for it.
#[derive(Serialize)]
struct Payload<'a> {
    message: &'a str,
}

impl Github {
    /// `PUT /repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}/dismissals`.
    pub async fn dismiss_pull_request_review(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &DismissPullRequestReview,
    ) -> Result<GithubReview, ProviderError> {
        let path = format!(
            "/repos/{}/{}/pulls/{}/reviews/{}/dismissals",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.pull_number.to_string()),
            path_segment(&input.review_id.to_string())
        );
        let payload = Payload {
            message: &input.message,
        };
        let answer: ReviewAnswer =
            send(self, http, token, Method::PUT, &path, &[], Some(&payload)).await?;
        Ok(answer.into())
    }
}
