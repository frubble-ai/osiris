//! `github.create_ref`: a new git reference, a branch or a tag, pointing at a commit.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send, transformers::git_ref::GitRefAnswer,
        types::GithubGitRef,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateRef {
    pub owner: String,
    pub repo: String,
    /// The full reference, starting with `refs/`, such as `refs/heads/feature`.
    #[serde(rename = "ref")]
    pub reference: String,
    /// The commit it points at.
    pub sha: String,
}

impl CreateRef {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

#[derive(Serialize)]
struct Body<'a> {
    #[serde(rename = "ref")]
    reference: &'a str,
    sha: &'a str,
}

impl Github {
    /// `POST /repos/{owner}/{repo}/git/refs`.
    pub async fn create_ref(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &CreateRef,
    ) -> Result<GithubGitRef, ProviderError> {
        let path = format!(
            "/repos/{}/{}/git/refs",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let body = Body {
            reference: &input.reference,
            sha: &input.sha,
        };
        let answer: GitRefAnswer =
            send(self, http, token, Method::POST, &path, &[], Some(&body)).await?;
        Ok(answer.into())
    }
}
