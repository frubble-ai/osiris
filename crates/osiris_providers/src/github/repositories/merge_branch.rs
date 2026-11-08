//! `github.merge_branch`: one branch merged into another.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send, transformers::commit::CommitAnswer,
        types::GithubMergeResult,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct MergeBranch {
    pub owner: String,
    pub repo: String,
    /// The branch merged into.
    pub base: String,
    /// The branch or commit SHA merged.
    pub head: String,
    /// The merge commit's message; GitHub writes one when left out.
    pub commit_message: Option<String>,
}

impl MergeBranch {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

#[derive(Serialize)]
struct Body<'a> {
    base: &'a str,
    head: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_message: Option<&'a str>,
}

impl Github {
    /// `POST /repos/{owner}/{repo}/merges`. GitHub answers the merge commit, or nothing (a 204)
    /// when the base already has the head; that answers `merged: false` and no `sha`.
    pub async fn merge_branch(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &MergeBranch,
    ) -> Result<GithubMergeResult, ProviderError> {
        let path = format!(
            "/repos/{}/{}/merges",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let body = Body {
            base: &input.base,
            head: &input.head,
            commit_message: input.commit_message.as_deref(),
        };
        let answer: Option<CommitAnswer> =
            send(self, http, token, Method::POST, &path, &[], Some(&body)).await?;
        Ok(match answer {
            Some(commit) => GithubMergeResult {
                sha: Some(commit.sha),
                merged: true,
                message: commit.commit.unwrap_or_default().message,
            },
            None => GithubMergeResult {
                sha: None,
                merged: false,
                message: "The base already contains the head, so there was nothing to merge."
                    .to_owned(),
            },
        })
    }
}
