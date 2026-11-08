//! `github.list_commit_comments`: the comments on a commit, a page at a time.

use crate::{
    ProviderError,
    github::{
        Github, paging::paging, path_segment::path_segment, request::send,
        transformers::commit_comment::CommitCommentAnswer, types::GithubCommitComment,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListCommitComments {
    pub owner: String,
    pub repo: String,
    pub commit_sha: String,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListCommitComments {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/commits/{commit_sha}/comments`.
    pub async fn list_commit_comments(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListCommitComments,
    ) -> Result<Vec<GithubCommitComment>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/commits/{}/comments",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.commit_sha)
        );
        let query = paging(input.per_page, input.page);
        let answer: Vec<CommitCommentAnswer> =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer.into_iter().map(GithubCommitComment::from).collect())
    }
}
