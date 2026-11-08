//! `github.get_issue_comment`: a comment on an issue, by id.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send,
        transformers::issue_comment::IssueCommentAnswer, types::GithubIssueComment,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct GetIssueComment {
    pub owner: String,
    pub repo: String,
    pub comment_id: u64,
}

impl GetIssueComment {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/issues/comments/{comment_id}`.
    pub async fn get_issue_comment(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &GetIssueComment,
    ) -> Result<GithubIssueComment, ProviderError> {
        let path = format!(
            "/repos/{}/{}/issues/comments/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.comment_id.to_string())
        );
        let answer: IssueCommentAnswer =
            send::<_, ()>(self, http, token, Method::GET, &path, &[], None).await?;
        Ok(answer.into())
    }
}
