//! `github.delete_pending_pull_request_review`: a pending review of a pull request, deleted; it answers the review deleted.

use crate::{
    ProviderError,
    github::{
        Github, github_id::GithubId, path_segment::path_segment, request::send,
        transformers::review::ReviewAnswer, types::GithubReview,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct DeletePendingPullRequestReview {
    pub owner: String,
    pub repo: String,
    pub pull_number: u64,
    pub review_id: GithubId,
}

impl DeletePendingPullRequestReview {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `DELETE /repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}`.
    pub async fn delete_pending_pull_request_review(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &DeletePendingPullRequestReview,
    ) -> Result<GithubReview, ProviderError> {
        let path = format!(
            "/repos/{}/{}/pulls/{}/reviews/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.pull_number.to_string()),
            path_segment(&input.review_id.to_string())
        );
        let answer: ReviewAnswer =
            send::<_, ()>(self, http, token, Method::DELETE, &path, &[], None).await?;
        Ok(answer.into())
    }
}
