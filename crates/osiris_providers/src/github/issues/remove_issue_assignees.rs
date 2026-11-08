//! `github.remove_issue_assignees`: people no longer assigned to an issue.

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
pub struct RemoveIssueAssignees {
    pub owner: String,
    pub repo: String,
    pub issue_number: u64,
    /// The logins of the people to unassign.
    pub assignees: Vec<String>,
}

impl RemoveIssueAssignees {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

#[derive(Serialize)]
struct RemoveIssueAssigneesBody<'a> {
    assignees: &'a [String],
}

impl Github {
    /// `DELETE /repos/{owner}/{repo}/issues/{issue_number}/assignees`, answering the issue.
    pub async fn remove_issue_assignees(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &RemoveIssueAssignees,
    ) -> Result<GithubIssue, ProviderError> {
        let path = format!(
            "/repos/{}/{}/issues/{}/assignees",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.issue_number.to_string())
        );
        let body = RemoveIssueAssigneesBody {
            assignees: &input.assignees,
        };
        let answer: IssueAnswer =
            send(self, http, token, Method::DELETE, &path, &[], Some(&body)).await?;
        Ok(answer.into())
    }
}
