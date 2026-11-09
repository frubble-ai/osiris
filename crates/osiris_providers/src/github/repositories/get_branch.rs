//! `github.get_branch`: a branch, by name.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send, transformers::branch::BranchAnswer,
        types::GithubBranch,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct GetBranch {
    pub owner: String,
    pub repo: String,
    pub branch: String,
}

impl GetBranch {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/branches/{branch}`.
    pub async fn get_branch(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &GetBranch,
    ) -> Result<GithubBranch, ProviderError> {
        let path = format!(
            "/repos/{}/{}/branches/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.branch)
        );
        let answer: BranchAnswer =
            send::<_, ()>(self, http, token, Method::GET, &path, &[], None).await?;
        Ok(answer.into())
    }
}
