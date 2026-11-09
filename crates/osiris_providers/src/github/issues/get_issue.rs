//! `github.get_issue`: an issue, by number.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send, transformers::issue::IssueAnswer,
        types::GithubIssue,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct GetIssue {
    pub owner: String,
    pub repo: String,
    pub issue_number: u64,
}

impl GetIssue {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/issues/{issue_number}`.
    pub async fn get_issue(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &GetIssue,
    ) -> Result<GithubIssue, ProviderError> {
        let path = format!(
            "/repos/{}/{}/issues/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.issue_number.to_string())
        );
        let answer: IssueAnswer =
            send::<_, ()>(self, http, token, Method::GET, &path, &[], None).await?;
        Ok(answer.into())
    }
}
