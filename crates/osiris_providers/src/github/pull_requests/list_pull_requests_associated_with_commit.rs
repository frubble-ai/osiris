//! `github.list_pull_requests_associated_with_commit`: the pull requests a commit belongs to, a
//! page at a time.

use crate::{
    ProviderError,
    github::{
        Github, paging::paging, path_segment::path_segment, request::send,
        transformers::pull_request::PullRequestAnswer, types::GithubPullRequest,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListPullRequestsAssociatedWithCommit {
    pub owner: String,
    pub repo: String,
    pub commit_sha: String,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListPullRequestsAssociatedWithCommit {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/commits/{commit_sha}/pulls`.
    pub async fn list_pull_requests_associated_with_commit(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListPullRequestsAssociatedWithCommit,
    ) -> Result<Vec<GithubPullRequest>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/commits/{}/pulls",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.commit_sha)
        );
        let query = paging(input.per_page, input.page);
        let answer: Vec<PullRequestAnswer> =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer.into_iter().map(GithubPullRequest::from).collect())
    }
}
