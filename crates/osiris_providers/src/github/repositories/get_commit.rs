//! `github.get_commit`: a commit, with its stats and the files it changed.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send, transformers::commit::CommitAnswer,
        types::GithubCommit,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct GetCommit {
    pub owner: String,
    pub repo: String,
    /// A commit SHA, a branch name or a tag name.
    #[serde(rename = "ref")]
    pub reference: String,
}

impl GetCommit {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/commits/{ref}`.
    pub async fn get_commit(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &GetCommit,
    ) -> Result<GithubCommit, ProviderError> {
        let path = format!(
            "/repos/{}/{}/commits/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.reference)
        );
        let answer: CommitAnswer =
            send::<_, ()>(self, http, token, Method::GET, &path, &[], None).await?;
        Ok(answer.into())
    }
}
