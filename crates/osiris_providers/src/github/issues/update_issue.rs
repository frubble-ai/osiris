//! `github.update_issue`: an issue's title, text, state, assignees, labels or milestone changed.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send, transformers::issue::IssueAnswer,
        types::GithubIssue,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Only the fields given are changed.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct UpdateIssue {
    pub owner: String,
    pub repo: String,
    pub issue_number: u64,
    pub title: Option<String>,
    /// The issue's text, in Markdown.
    pub body: Option<String>,
    /// `open` or `closed`.
    pub state: Option<String>,
    /// The logins of the people assigned to it, in place of those assigned now.
    pub assignees: Option<Vec<String>>,
    /// The names of its labels, in place of those it has now.
    pub labels: Option<Vec<String>>,
    /// The number of the milestone to put it in.
    pub milestone: Option<u64>,
}

impl UpdateIssue {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

#[derive(Serialize)]
struct UpdateIssueBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assignees: Option<&'a [String]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<&'a [String]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    milestone: Option<u64>,
}

impl Github {
    /// `PATCH /repos/{owner}/{repo}/issues/{issue_number}`.
    pub async fn update_issue(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &UpdateIssue,
    ) -> Result<GithubIssue, ProviderError> {
        let path = format!(
            "/repos/{}/{}/issues/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.issue_number.to_string())
        );
        let body = UpdateIssueBody {
            title: input.title.as_deref(),
            body: input.body.as_deref(),
            state: input.state.as_deref(),
            assignees: input.assignees.as_deref(),
            labels: input.labels.as_deref(),
            milestone: input.milestone,
        };
        let answer: IssueAnswer =
            send(self, http, token, Method::PATCH, &path, &[], Some(&body)).await?;
        Ok(answer.into())
    }
}
