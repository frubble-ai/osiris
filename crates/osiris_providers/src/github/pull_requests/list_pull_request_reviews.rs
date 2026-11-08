//! `github.list_pull_request_reviews`: the reviews of a pull request, a page at a time.

use crate::{
    ProviderError,
    github::{
        Github, paging::paging, path_segment::path_segment, request::send,
        transformers::review::ReviewAnswer, types::GithubReview,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListPullRequestReviews {
    pub owner: String,
    pub repo: String,
    pub pull_number: u64,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListPullRequestReviews {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/pulls/{pull_number}/reviews`.
    pub async fn list_pull_request_reviews(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListPullRequestReviews,
    ) -> Result<Vec<GithubReview>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/pulls/{}/reviews",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.pull_number.to_string())
        );
        let query = paging(input.per_page, input.page);
        let answer: Vec<ReviewAnswer> =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer.into_iter().map(GithubReview::from).collect())
    }
}
