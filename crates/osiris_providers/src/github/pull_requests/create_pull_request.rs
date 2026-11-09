//! `github.create_pull_request`: a new pull request, from one branch into another.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send,
        transformers::pull_request::PullRequestAnswer, types::GithubPullRequest,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreatePullRequest {
    pub owner: String,
    pub repo: String,
    pub title: String,
    /// The branch the changes come from. A branch of another account's fork is named
    /// `account:branch`.
    pub head: String,
    /// The branch the changes go into.
    pub base: String,
    pub body: Option<String>,
    /// Whether the pull request is opened as a draft.
    pub draft: Option<bool>,
    /// Whether the base repository's maintainers may push to the head branch.
    pub maintainer_can_modify: Option<bool>,
}

impl CreatePullRequest {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

/// The pull request as GitHub asks for it.
#[derive(Serialize)]
struct Payload<'a> {
    title: &'a str,
    head: &'a str,
    base: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    draft: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintainer_can_modify: Option<bool>,
}

impl Github {
    /// `POST /repos/{owner}/{repo}/pulls`.
    pub async fn create_pull_request(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &CreatePullRequest,
    ) -> Result<GithubPullRequest, ProviderError> {
        let path = format!(
            "/repos/{}/{}/pulls",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let payload = Payload {
            title: &input.title,
            head: &input.head,
            base: &input.base,
            body: input.body.as_deref(),
            draft: input.draft,
            maintainer_can_modify: input.maintainer_can_modify,
        };
        let answer: PullRequestAnswer =
            send(self, http, token, Method::POST, &path, &[], Some(&payload)).await?;
        Ok(answer.into())
    }
}
