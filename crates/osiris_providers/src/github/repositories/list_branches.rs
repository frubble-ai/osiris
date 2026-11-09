//! `github.list_branches`: a repository's branches, a page at a time.

use crate::{
    ProviderError,
    github::{
        Github, paging::paging, path_segment::path_segment, request::send,
        transformers::branch::BranchAnswer, types::GithubBranch,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListBranches {
    pub owner: String,
    pub repo: String,
    /// Only protected branches (`true`) or only unprotected ones (`false`); both when left out.
    pub protected: Option<bool>,
    /// At most 100.
    pub per_page: Option<u32>,
    /// From 1.
    pub page: Option<u32>,
}

impl ListBranches {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/branches`.
    pub async fn list_branches(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListBranches,
    ) -> Result<Vec<GithubBranch>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/branches",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let mut query = paging(input.per_page, input.page);
        if let Some(protected) = input.protected {
            query.push(("protected", protected.to_string()));
        }
        let answer: Vec<BranchAnswer> =
            send::<_, ()>(self, http, token, Method::GET, &path, &query, None).await?;
        Ok(answer.into_iter().map(GithubBranch::from).collect())
    }
}
