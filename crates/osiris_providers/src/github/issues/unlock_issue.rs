//! `github.unlock_issue`: an issue's conversation unlocked.

use crate::{
    ProviderError,
    github::{Github, path_segment::path_segment, request::send, types::GithubIssueLock},
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct UnlockIssue {
    pub owner: String,
    pub repo: String,
    pub issue_number: u64,
}

impl UnlockIssue {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `DELETE /repos/{owner}/{repo}/issues/{issue_number}/lock`.
    pub async fn unlock_issue(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &UnlockIssue,
    ) -> Result<GithubIssueLock, ProviderError> {
        let path = format!(
            "/repos/{}/{}/issues/{}/lock",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.issue_number.to_string())
        );
        send::<(), ()>(self, http, token, Method::DELETE, &path, &[], None).await?;
        Ok(GithubIssueLock { locked: false })
    }
}
