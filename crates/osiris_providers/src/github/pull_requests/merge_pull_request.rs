//! `github.merge_pull_request`: a pull request merged into its base branch.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send,
        transformers::merge_result::MergeResultAnswer, types::GithubMergeResult,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// How a pull request's commits reach its base branch: a merge commit, one squashed commit, or
/// each commit rebased onto the base branch.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MergeMethod {
    Merge,
    Squash,
    Rebase,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct MergePullRequest {
    pub owner: String,
    pub repo: String,
    pub pull_number: u64,
    /// The title of the merge commit; GitHub writes one when left out.
    pub commit_title: Option<String>,
    /// The text added to the merge commit's message.
    pub commit_message: Option<String>,
    /// The head commit the pull request must be at for the merge to go ahead.
    pub sha: Option<String>,
    /// The repository's default method when left out.
    pub merge_method: Option<MergeMethod>,
}

impl MergePullRequest {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

/// The merge as GitHub asks for it.
#[derive(Serialize)]
struct Payload<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_message: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sha: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_method: Option<MergeMethod>,
}

impl Github {
    /// `PUT /repos/{owner}/{repo}/pulls/{pull_number}/merge`.
    pub async fn merge_pull_request(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &MergePullRequest,
    ) -> Result<GithubMergeResult, ProviderError> {
        let path = format!(
            "/repos/{}/{}/pulls/{}/merge",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.pull_number.to_string())
        );
        let payload = Payload {
            commit_title: input.commit_title.as_deref(),
            commit_message: input.commit_message.as_deref(),
            sha: input.sha.as_deref(),
            merge_method: input.merge_method,
        };
        let answer: MergeResultAnswer =
            send(self, http, token, Method::PUT, &path, &[], Some(&payload)).await?;
        Ok(answer.into())
    }
}
