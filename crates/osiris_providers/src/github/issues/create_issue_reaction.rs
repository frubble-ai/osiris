//! `github.create_issue_reaction`: a reaction to an issue.

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
pub struct CreateIssueReaction {
    pub owner: String,
    pub repo: String,
    pub issue_number: u64,
    /// The reaction: `+1`, `-1`, `laugh`, `confused`, `heart`, `hooray`, `rocket` or `eyes`.
    pub content: String,
}

impl CreateIssueReaction {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

#[derive(Serialize)]
struct CreateIssueReactionBody<'a> {
    content: &'a str,
}

impl Github {
    /// `POST /repos/{owner}/{repo}/issues/{issue_number}/reactions`.
    pub async fn create_issue_reaction(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &CreateIssueReaction,
    ) -> Result<GithubReaction, ProviderError> {
        let path = format!(
            "/repos/{}/{}/issues/{}/reactions",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.issue_number.to_string())
        );
        let body = CreateIssueReactionBody {
            content: &input.content,
        };
        let answer: ReactionAnswer =
            send(self, http, token, Method::POST, &path, &[], Some(&body)).await?;
        Ok(answer.into())
    }
}
