//! `github.get_pull_request_review`: a review of a pull request, by id.

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
pub struct GetPullRequestReview {
    pub owner: String,
    pub repo: String,
    pub pull_number: u64,
    pub review_id: GithubId,
}

impl GetPullRequestReview {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}`.
    pub async fn get_pull_request_review(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &GetPullRequestReview,
    ) -> Result<GithubReview, ProviderError> {
        let path = format!(
            "/repos/{}/{}/pulls/{}/reviews/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.pull_number.to_string()),
            path_segment(&input.review_id.to_string())
        );
        let answer: ReviewAnswer =
            send::<_, ()>(self, http, token, Method::GET, &path, &[], None).await?;
        Ok(answer.into())
    }
}
