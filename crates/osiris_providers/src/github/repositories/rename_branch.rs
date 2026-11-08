//! `github.rename_branch`: a branch given a new name.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send, transformers::branch::BranchAnswer,
        types::GithubBranch,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct RenameBranch {
    pub owner: String,
    pub repo: String,
    /// The branch's current name.
    pub branch: String,
    pub new_name: String,
}

impl RenameBranch {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

#[derive(Serialize)]
struct Body<'a> {
    new_name: &'a str,
}

impl Github {
    /// `POST /repos/{owner}/{repo}/branches/{branch}/rename`.
    pub async fn rename_branch(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &RenameBranch,
    ) -> Result<GithubBranch, ProviderError> {
        let path = format!(
            "/repos/{}/{}/branches/{}/rename",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.branch)
        );
        let body = Body {
            new_name: &input.new_name,
        };
        let answer: BranchAnswer =
            send(self, http, token, Method::POST, &path, &[], Some(&body)).await?;
        Ok(answer.into())
    }
}
