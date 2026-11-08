//! `github.get_pull_request`: a pull request, by number.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send,
        transformers::pull_request::PullRequestAnswer, types::GithubPullRequest,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct GetPullRequest {
    pub owner: String,
    pub repo: String,
    pub pull_number: u64,
}

impl GetPullRequest {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/pulls/{pull_number}`.
    pub async fn get_pull_request(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &GetPullRequest,
    ) -> Result<GithubPullRequest, ProviderError> {
        let path = format!(
            "/repos/{}/{}/pulls/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.pull_number.to_string())
        );
        let answer: PullRequestAnswer =
            send::<_, ()>(self, http, token, Method::GET, &path, &[], None).await?;
        Ok(answer.into())
    }
}
