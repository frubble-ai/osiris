//! `github.update_ref`: a branch's or a tag's git reference pointed at another commit.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, repositories::git_ref_path::branch_or_tag_ref_path,
        request::send, transformers::git_ref::GitRefAnswer, types::GithubGitRef,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct UpdateRef {
    pub owner: String,
    pub repo: String,
    /// The reference without `refs/`, such as `heads/main` or `tags/v1.0.0`.
    #[serde(rename = "ref")]
    pub reference: String,
    /// The commit it points at from now on.
    pub sha: String,
    /// Whether to move it even when the new commit doesn't descend from the old one.
    pub force: Option<bool>,
}

impl UpdateRef {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

#[derive(Serialize)]
struct Body<'a> {
    sha: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    force: Option<bool>,
}

impl Github {
    /// `PATCH /repos/{owner}/{repo}/git/refs/{ref}`. A reference that is not a branch's or a
    /// tag's is refused before GitHub is asked.
    pub async fn update_ref(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &UpdateRef,
    ) -> Result<GithubGitRef, ProviderError> {
        let path = format!(
            "/repos/{}/{}/git/refs/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            branch_or_tag_ref_path(&input.reference)?
        );
        let body = Body {
            sha: &input.sha,
            force: input.force,
        };
        let answer: GitRefAnswer =
            send(self, http, token, Method::PATCH, &path, &[], Some(&body)).await?;
        Ok(answer.into())
    }
}
