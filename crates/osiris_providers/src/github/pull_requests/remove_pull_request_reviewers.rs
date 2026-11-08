//! `github.remove_pull_request_reviewers`: users and teams removed from those a pull request's review is requested from.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send,
        transformers::review_request::ReviewRequestAnswer, types::GithubReviewRequest,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct RemovePullRequestReviewers {
    pub owner: String,
    pub repo: String,
    pub pull_number: u64,
    /// The users, by login.
    pub reviewers: Option<Vec<String>>,
    /// The teams, by slug.
    pub team_reviewers: Option<Vec<String>>,
}

impl RemovePullRequestReviewers {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

/// The reviewers as GitHub asks for them.
#[derive(Serialize)]
struct Payload<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    reviewers: Option<&'a [String]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    team_reviewers: Option<&'a [String]>,
}

impl Github {
    /// `DELETE /repos/{owner}/{repo}/pulls/{pull_number}/requested_reviewers`.
    pub async fn remove_pull_request_reviewers(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &RemovePullRequestReviewers,
    ) -> Result<GithubReviewRequest, ProviderError> {
        let path = format!(
            "/repos/{}/{}/pulls/{}/requested_reviewers",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.pull_number.to_string())
        );
        let payload = Payload {
            reviewers: input.reviewers.as_deref(),
            team_reviewers: input.team_reviewers.as_deref(),
        };
        let answer: ReviewRequestAnswer = send(
            self,
            http,
            token,
            Method::DELETE,
            &path,
            &[],
            Some(&payload),
        )
        .await?;
        Ok(answer.into())
    }
}
