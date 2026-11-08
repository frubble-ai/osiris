//! `github.delete_ref`: a branch's or a tag's git reference deleted.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, repositories::git_ref_path::branch_or_tag_ref_path,
        request::send, types::GithubAcknowledgement,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct DeleteRef {
    pub owner: String,
    pub repo: String,
    /// The reference without `refs/`, such as `heads/feature` or `tags/v1.0.0`.
    #[serde(rename = "ref")]
    pub reference: String,
}

impl DeleteRef {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `DELETE /repos/{owner}/{repo}/git/refs/{ref}`. A reference that is not a branch's or a
    /// tag's is refused before GitHub is asked.
    pub async fn delete_ref(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &DeleteRef,
    ) -> Result<GithubAcknowledgement, ProviderError> {
        let path = format!(
            "/repos/{}/{}/git/refs/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            branch_or_tag_ref_path(&input.reference)?
        );
        send::<(), ()>(self, http, token, Method::DELETE, &path, &[], None).await?;
        Ok(GithubAcknowledgement { ok: true })
    }
}
