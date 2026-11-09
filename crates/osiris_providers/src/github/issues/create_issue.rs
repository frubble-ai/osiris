//! `github.create_issue`: a new issue in a repository.

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

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateIssue {
    pub owner: String,
    pub repo: String,
    pub title: String,
    /// The issue's text, in Markdown.
    pub body: Option<String>,
    /// The logins of the people to assign it to.
    pub assignees: Option<Vec<String>>,
    /// The names of the labels to put on it.
    pub labels: Option<Vec<String>>,
    /// The number of the milestone to put it in.
    pub milestone: Option<u64>,
}

impl CreateIssue {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

#[derive(Serialize)]
struct CreateIssueBody<'a> {
    title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    assignees: Option<&'a [String]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<&'a [String]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    milestone: Option<u64>,
}

impl Github {
    /// `POST /repos/{owner}/{repo}/issues`.
    pub async fn create_issue(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &CreateIssue,
    ) -> Result<GithubIssue, ProviderError> {
        let path = format!(
            "/repos/{}/{}/issues",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let body = CreateIssueBody {
            title: &input.title,
            body: input.body.as_deref(),
            assignees: input.assignees.as_deref(),
            labels: input.labels.as_deref(),
            milestone: input.milestone,
        };
        let answer: IssueAnswer =
            send(self, http, token, Method::POST, &path, &[], Some(&body)).await?;
        Ok(answer.into())
    }
}
