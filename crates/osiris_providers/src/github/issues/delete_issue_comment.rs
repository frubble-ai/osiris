//! `github.delete_issue_comment`: a comment on an issue deleted.

use crate::{
    ProviderError,
    github::{Github, path_segment::path_segment, request::send, types::GithubAcknowledgement},
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct DeleteIssueComment {
    pub owner: String,
    pub repo: String,
    pub comment_id: u64,
}

impl DeleteIssueComment {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `DELETE /repos/{owner}/{repo}/issues/comments/{comment_id}`.
    pub async fn delete_issue_comment(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &DeleteIssueComment,
    ) -> Result<GithubAcknowledgement, ProviderError> {
        let path = format!(
            "/repos/{}/{}/issues/comments/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.comment_id.to_string())
        );
        send::<(), ()>(self, http, token, Method::DELETE, &path, &[], None).await?;
        Ok(GithubAcknowledgement { ok: true })
    }
}
