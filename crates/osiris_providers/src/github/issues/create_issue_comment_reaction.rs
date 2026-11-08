//! `github.create_issue_comment_reaction`: a reaction to a comment on an issue.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send, transformers::reaction::ReactionAnswer,
        types::GithubReaction,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateIssueCommentReaction {
    pub owner: String,
    pub repo: String,
    pub comment_id: u64,
    /// The reaction: `+1`, `-1`, `laugh`, `confused`, `heart`, `hooray`, `rocket` or `eyes`.
    pub content: String,
}

impl CreateIssueCommentReaction {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

#[derive(Serialize)]
struct CreateIssueCommentReactionBody<'a> {
    content: &'a str,
}

impl Github {
    /// `POST /repos/{owner}/{repo}/issues/comments/{comment_id}/reactions`.
    pub async fn create_issue_comment_reaction(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &CreateIssueCommentReaction,
    ) -> Result<GithubReaction, ProviderError> {
        let path = format!(
            "/repos/{}/{}/issues/comments/{}/reactions",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.comment_id.to_string())
        );
        let body = CreateIssueCommentReactionBody {
            content: &input.content,
        };
        let answer: ReactionAnswer =
            send(self, http, token, Method::POST, &path, &[], Some(&body)).await?;
        Ok(answer.into())
    }
}
