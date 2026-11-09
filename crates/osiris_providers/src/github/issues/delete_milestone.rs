//! `github.delete_milestone`: a milestone deleted from a repository.

use crate::{
    ProviderError,
    github::{Github, path_segment::path_segment, request::send, types::GithubAcknowledgement},
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct DeleteMilestone {
    pub owner: String,
    pub repo: String,
    pub milestone_number: u64,
}

impl DeleteMilestone {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `DELETE /repos/{owner}/{repo}/milestones/{milestone_number}`.
    pub async fn delete_milestone(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &DeleteMilestone,
    ) -> Result<GithubAcknowledgement, ProviderError> {
        let path = format!(
            "/repos/{}/{}/milestones/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.milestone_number.to_string())
        );
        send::<(), ()>(self, http, token, Method::DELETE, &path, &[], None).await?;
        Ok(GithubAcknowledgement { ok: true })
    }
}
