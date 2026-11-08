//! `github.remove_issue_label`: one label taken off an issue.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send, transformers::label::LabelAnswer,
        types::GithubLabel,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct RemoveIssueLabel {
    pub owner: String,
    pub repo: String,
    pub issue_number: u64,
    /// The name of the label to take off.
    pub label: String,
}

impl RemoveIssueLabel {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `DELETE /repos/{owner}/{repo}/issues/{issue_number}/labels/{label}`, answering the labels
    /// the issue keeps.
    pub async fn remove_issue_label(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &RemoveIssueLabel,
    ) -> Result<Vec<GithubLabel>, ProviderError> {
        let path = format!(
            "/repos/{}/{}/issues/{}/labels/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.issue_number.to_string()),
            path_segment(&input.label)
        );
        let answer: Vec<LabelAnswer> =
            send::<_, ()>(self, http, token, Method::DELETE, &path, &[], None).await?;
        Ok(answer.into_iter().map(GithubLabel::from).collect())
    }
}
