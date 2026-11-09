//! `github.list_pull_request_requested_reviewers`: the users and teams a pull request's review is
//! requested from.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send,
        transformers::requested_reviewers::RequestedReviewersAnswer,
        types::GithubRequestedReviewers,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListPullRequestRequestedReviewers {
    pub owner: String,
    pub repo: String,
    pub pull_number: u64,
}

impl ListPullRequestRequestedReviewers {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/pulls/{pull_number}/requested_reviewers`.
    pub async fn list_pull_request_requested_reviewers(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListPullRequestRequestedReviewers,
    ) -> Result<GithubRequestedReviewers, ProviderError> {
        let path = format!(
            "/repos/{}/{}/pulls/{}/requested_reviewers",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.pull_number.to_string())
        );
        let answer: RequestedReviewersAnswer =
            send::<_, ()>(self, http, token, Method::GET, &path, &[], None).await?;
        Ok(answer.into())
    }
}
