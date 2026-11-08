//! `github.check_pull_request_merged`: whether a pull request has been merged.

use crate::{
    ProviderError,
    github::{Github, path_segment::path_segment, request::send, types::GithubMergedCheck},
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CheckPullRequestMerged {
    pub owner: String,
    pub repo: String,
    pub pull_number: u64,
}

impl CheckPullRequestMerged {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/pulls/{pull_number}/merge`. GitHub answers 204, with no
    /// content, when the pull request has been merged, and 404 when it has not.
    pub async fn check_pull_request_merged(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &CheckPullRequestMerged,
    ) -> Result<GithubMergedCheck, ProviderError> {
        let path = format!(
            "/repos/{}/{}/pulls/{}/merge",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.pull_number.to_string())
        );
        match send::<(), ()>(self, http, token, Method::GET, &path, &[], None).await {
            Ok(()) => Ok(GithubMergedCheck { merged: true }),
            Err(error) if error.is_not_found() => Ok(GithubMergedCheck { merged: false }),
            Err(error) => Err(error),
        }
    }
}
