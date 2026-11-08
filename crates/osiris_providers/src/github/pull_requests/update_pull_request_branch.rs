//! `github.update_pull_request_branch`: a pull request's branch brought up to date with the latest
//! changes of its base branch.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send,
        transformers::pull_request_branch_update::PullRequestBranchUpdateAnswer, trimmed::trimmed,
        types::GithubPullRequestBranchUpdate,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct UpdatePullRequestBranch {
    pub owner: String,
    pub repo: String,
    pub pull_number: u64,
    /// The head commit the branch is expected to be at. GitHub refuses the update when the
    /// branch has moved on since.
    pub expected_head_sha: Option<String>,
}

impl UpdatePullRequestBranch {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

/// The update as GitHub asks for it.
#[derive(Serialize)]
struct Payload<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expected_head_sha: Option<&'a str>,
}

impl Github {
    /// `PUT /repos/{owner}/{repo}/pulls/{pull_number}/update-branch`.
    pub async fn update_pull_request_branch(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &UpdatePullRequestBranch,
    ) -> Result<GithubPullRequestBranchUpdate, ProviderError> {
        let path = format!(
            "/repos/{}/{}/pulls/{}/update-branch",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.pull_number.to_string())
        );
        let payload = Payload {
            expected_head_sha: trimmed(input.expected_head_sha.as_deref()),
        };
        let answer: PullRequestBranchUpdateAnswer =
            send(self, http, token, Method::PUT, &path, &[], Some(&payload)).await?;
        Ok(answer.into())
    }
}
