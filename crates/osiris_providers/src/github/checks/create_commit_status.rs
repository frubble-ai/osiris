//! `github.create_commit_status`: sets a status on a commit.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send,
        transformers::commit_status::CommitStatusAnswer, types::GithubCommitStatus,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateCommitStatus {
    pub owner: String,
    pub repo: String,
    /// The commit's full SHA.
    pub sha: String,
    /// `error`, `failure`, `pending` or `success`.
    pub state: String,
    /// The name that tells this status apart from other services' statuses on the commit.
    /// GitHub uses `default` when it is left out.
    pub context: Option<String>,
    /// The page the status links to, such as a build's output.
    pub target_url: Option<String>,
    /// A short description of the status.
    pub description: Option<String>,
}

impl CreateCommitStatus {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

/// The status, as GitHub reads it.
#[derive(Serialize)]
struct Body<'a> {
    state: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
}

impl Github {
    /// `POST /repos/{owner}/{repo}/statuses/{sha}`.
    pub async fn create_commit_status(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &CreateCommitStatus,
    ) -> Result<GithubCommitStatus, ProviderError> {
        let path = format!(
            "/repos/{}/{}/statuses/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.sha)
        );
        let body = Body {
            state: &input.state,
            context: input.context.as_deref(),
            target_url: input.target_url.as_deref(),
            description: input.description.as_deref(),
        };
        let answer: CommitStatusAnswer =
            send(self, http, token, Method::POST, &path, &[], Some(&body)).await?;
        Ok(answer.into())
    }
}
