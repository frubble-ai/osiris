//! `github.add_issue_labels`: labels added to an issue, beside those it has.

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
pub struct AddIssueLabels {
    pub owner: String,
    pub repo: String,
    pub issue_number: u64,
    /// The names of the labels to add.
    pub labels: Vec<String>,
}

impl AddIssueLabels {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

#[derive(Serialize)]
struct AddIssueLabelsBody<'a> {
    labels: &'a [String],
}

impl Github {
    /// `POST /repos/{owner}/{repo}/issues/{issue_number}/labels`, answering every label the issue
    /// has afterwards.
    pub async fn add_issue_labels(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &AddIssueLabels,
    ) -> Result<Vec<GithubLabel>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/issues/{}/labels",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.issue_number.to_string())
        );
        let body = AddIssueLabelsBody {
            labels: &input.labels,
        };
        let answer: Vec<LabelAnswer> =
            send(self, http, token, Method::POST, &path, &[], Some(&body)).await?;
        Ok(answer.into_iter().map(GithubLabel::from).collect())
    }
}
