//! `github.lock_issue`: an issue's conversation locked, so only collaborators can comment.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send, trimmed::trimmed, types::GithubIssueLock,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct LockIssue {
    pub owner: String,
    pub repo: String,
    pub issue_number: u64,
    /// Why: `off-topic`, `too heated`, `resolved` or `spam`.
    pub lock_reason: Option<String>,
}

impl LockIssue {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

#[derive(Serialize)]
struct LockIssueBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    lock_reason: Option<&'a str>,
}

impl Github {
    /// `PUT /repos/{owner}/{repo}/issues/{issue_number}/lock`.
    pub async fn lock_issue(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &LockIssue,
    ) -> Result<GithubIssueLock, ProviderError> {
        let path = format!(
            "/repos/{}/{}/issues/{}/lock",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.issue_number.to_string())
        );
        let body = LockIssueBody {
            lock_reason: trimmed(input.lock_reason.as_deref()),
        };
        send::<(), _>(self, http, token, Method::PUT, &path, &[], Some(&body)).await?;
        Ok(GithubIssueLock { locked: true })
    }
}
