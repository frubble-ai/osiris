//! `github.list_matching_refs`: every git reference that starts with a prefix. GitHub doesn't page
//! it: every match comes at once.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, repositories::git_ref_path::git_ref_path,
        request::send, transformers::git_ref::GitRefAnswer, types::GithubGitRef,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ListMatchingRefs {
    pub owner: String,
    pub repo: String,
    /// The prefix to match, without `refs/`, such as `heads/feature`.
    #[serde(rename = "ref")]
    pub reference: String,
}

impl ListMatchingRefs {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/git/matching-refs/{ref}`.
    pub async fn list_matching_refs(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &ListMatchingRefs,
    ) -> Result<Vec<GithubGitRef>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/git/matching-refs/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            git_ref_path(&input.reference)
        );
        let answer: Vec<GitRefAnswer> =
            send::<_, ()>(self, http, token, Method::GET, &path, &[], None).await?;
        Ok(answer.into_iter().map(GithubGitRef::from).collect())
    }
}
