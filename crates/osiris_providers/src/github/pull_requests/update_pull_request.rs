//! `github.update_pull_request`: a pull request's title, text, state, base branch or whether
//! maintainers may push to it, changed.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send,
        transformers::pull_request::PullRequestAnswer, trimmed::trimmed, types::GithubPullRequest,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// The state a pull request is put in: open, or closed without merging.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PullRequestState {
    Open,
    Closed,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct UpdatePullRequest {
    pub owner: String,
    pub repo: String,
    pub pull_number: u64,
    pub title: Option<String>,
    pub body: Option<String>,
    pub state: Option<PullRequestState>,
    /// The branch the changes go into.
    pub base: Option<String>,
    /// Whether the base repository's maintainers may push to the head branch.
    pub maintainer_can_modify: Option<bool>,
}

impl UpdatePullRequest {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

/// The change as GitHub asks for it: only what is given.
#[derive(Serialize)]
struct Payload<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PullRequestState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintainer_can_modify: Option<bool>,
}

impl Github {
    /// `PATCH /repos/{owner}/{repo}/pulls/{pull_number}`.
    pub async fn update_pull_request(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &UpdatePullRequest,
    ) -> Result<GithubPullRequest, ProviderError> {
        let path = format!(
            "/repos/{}/{}/pulls/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.pull_number.to_string())
        );
        let payload = Payload {
            title: input.title.as_deref(),
            body: input.body.as_deref(),
            state: input.state,
            base: trimmed(input.base.as_deref()),
            maintainer_can_modify: input.maintainer_can_modify,
        };
        let answer: PullRequestAnswer =
            send(self, http, token, Method::PATCH, &path, &[], Some(&payload)).await?;
        Ok(answer.into())
    }
}
