//! `github.compare_commits`: two commits compared, with the commits and files between them.

use crate::{
    ProviderError,
    github::{
        Github, paging::paging, path_segment::path_segment, request::send,
        transformers::commit_comparison::CommitComparisonAnswer, types::GithubCommitComparison,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CompareCommits {
    pub owner: String,
    pub repo: String,
    /// `base...head`, each a branch, a tag or a commit SHA; `owner:branch` names another fork's.
    pub basehead: String,
    /// At most 100; pages the commits between them.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl CompareCommits {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/compare/{basehead}`.
    pub async fn compare_commits(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &CompareCommits,
    ) -> Result<GithubCommitComparison, ProviderError> {
        let path = format!(
            "/repos/{}/{}/compare/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.basehead)
        );
        let query = paging(input.per_page, input.page);
        let answer: CommitComparisonAnswer =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer.into())
    }
}
