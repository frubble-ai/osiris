//! `github.sync_fork_branch_with_upstream`: a fork's branch brought up to date with its upstream.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send, transformers::fork_sync::ForkSyncAnswer,
        types::GithubForkSync,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct SyncForkBranchWithUpstream {
    /// The fork's owner.
    pub owner: String,
    /// The fork's name.
    pub repo: String,
    /// The fork's branch to bring up to date.
    pub branch: String,
}

impl SyncForkBranchWithUpstream {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

#[derive(Serialize)]
struct Body<'a> {
    branch: &'a str,
}

impl Github {
    /// `POST /repos/{owner}/{repo}/merge-upstream`.
    pub async fn sync_fork_branch_with_upstream(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &SyncForkBranchWithUpstream,
    ) -> Result<GithubForkSync, ProviderError> {
        let path = format!(
            "/repos/{}/{}/merge-upstream",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let body = Body {
            branch: &input.branch,
        };
        let answer: ForkSyncAnswer =
            send(self, http, token, Method::POST, &path, &[], Some(&body)).await?;
        Ok(answer.into())
    }
}
