//! `github.get_commit_statuses`: the statuses set on a commit, newest first, a page at a time.

use crate::{
    ProviderError,
    github::{
        Github, paging::paging, path_segment::path_segment, request::send,
        transformers::commit_status::CommitStatusAnswer, types::GithubCommitStatus,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct GetCommitStatuses {
    pub owner: String,
    pub repo: String,
    /// A commit SHA, a branch name or a tag name.
    #[serde(rename = "ref")]
    pub git_ref: String,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl GetCommitStatuses {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/commits/{ref}/statuses`.
    pub async fn get_commit_statuses(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &GetCommitStatuses,
    ) -> Result<Vec<GithubCommitStatus>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/commits/{}/statuses",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.git_ref)
        );
        let query = paging(input.per_page, input.page);
        let answer: Vec<CommitStatusAnswer> =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer.into_iter().map(GithubCommitStatus::from).collect())
    }
}
