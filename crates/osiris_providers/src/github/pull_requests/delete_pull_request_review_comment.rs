//! `github.delete_pull_request_review_comment`: a comment on a pull request's diff, deleted.

use crate::{
    ProviderError,
    github::{
        Github, github_id::GithubId, path_segment::path_segment, request::send,
        types::GithubAcknowledgement,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct DeletePullRequestReviewComment {
    pub owner: String,
    pub repo: String,
    pub comment_id: GithubId,
}

impl DeletePullRequestReviewComment {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `DELETE /repos/{owner}/{repo}/pulls/comments/{comment_id}`, which GitHub answers with no
    /// content.
    pub async fn delete_pull_request_review_comment(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &DeletePullRequestReviewComment,
    ) -> Result<GithubAcknowledgement, ProviderError> {
        let path = format!(
            "/repos/{}/{}/pulls/comments/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.comment_id.to_string())
        );
        send::<(), ()>(self, http, token, Method::DELETE, &path, &[], None).await?;
        Ok(GithubAcknowledgement { ok: true })
    }
}
