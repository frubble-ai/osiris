//! `github.get_ref`: a branch's or a tag's git reference.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, repositories::git_ref_path::branch_or_tag_ref_path,
        request::send, transformers::git_ref::GitRefAnswer, types::GithubGitRef,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct GetRef {
    pub owner: String,
    pub repo: String,
    /// The reference without `refs/`, such as `heads/main` or `tags/v1.0.0`.
    #[serde(rename = "ref")]
    pub reference: String,
}

impl GetRef {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/git/ref/{ref}`. A reference that is not a branch's or a tag's
    /// is refused before GitHub is asked.
    pub async fn get_ref(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &GetRef,
    ) -> Result<GithubGitRef, ProviderError> {
        let path = format!(
            "/repos/{}/{}/git/ref/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            branch_or_tag_ref_path(&input.reference)?
        );
        let answer: GitRefAnswer =
            send::<_, ()>(self, http, token, Method::GET, &path, &[], None).await?;
        Ok(answer.into())
    }
}
