//! `github.list_pull_request_commits`: the commits on a pull request, a page at a time.

use crate::{
    ProviderError,
    github::{
        Github, paging::paging, path_segment::path_segment, request::send,
        transformers::commit::CommitAnswer, types::GithubCommit,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListPullRequestCommits {
    pub owner: String,
    pub repo: String,
    pub pull_number: u64,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListPullRequestCommits {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/pulls/{pull_number}/commits`.
    pub async fn list_pull_request_commits(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListPullRequestCommits,
    ) -> Result<Vec<GithubCommit>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/pulls/{}/commits",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.pull_number.to_string())
        );
        let query = paging(input.per_page, input.page);
        let answer: Vec<CommitAnswer> =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer.into_iter().map(GithubCommit::from).collect())
    }
}
