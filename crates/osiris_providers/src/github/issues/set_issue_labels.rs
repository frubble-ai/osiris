//! `github.set_issue_labels`: an issue's labels replaced with those given.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send, transformers::label::LabelAnswer,
        types::GithubLabel,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct SetIssueLabels {
    pub owner: String,
    pub repo: String,
    pub issue_number: u64,
    /// The names of the labels the issue keeps; every other label is taken off.
    pub labels: Vec<String>,
}

impl SetIssueLabels {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

#[derive(Serialize)]
struct SetIssueLabelsBody<'a> {
    labels: &'a [String],
}

impl Github {
    /// `PUT /repos/{owner}/{repo}/issues/{issue_number}/labels`.
    pub async fn set_issue_labels(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &SetIssueLabels,
    ) -> Result<Vec<GithubLabel>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/issues/{}/labels",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.issue_number.to_string())
        );
        let body = SetIssueLabelsBody {
            labels: &input.labels,
        };
        let answer: Vec<LabelAnswer> =
            send(self, http, token, Method::PUT, &path, &[], Some(&body)).await?;
        Ok(answer.into_iter().map(GithubLabel::from).collect())
    }
}
