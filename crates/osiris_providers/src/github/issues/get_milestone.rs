//! `github.get_milestone`: a milestone, by number.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send,
        transformers::milestone::MilestoneAnswer, types::GithubMilestone,
    },
};
use reqwest::Method;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct GetMilestone {
    pub owner: String,
    pub repo: String,
    pub milestone_number: u64,
}

impl GetMilestone {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

impl Github {
    /// `GET /repos/{owner}/{repo}/milestones/{milestone_number}`.
    pub async fn get_milestone(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &GetMilestone,
    ) -> Result<GithubMilestone, ProviderError> {
        let path = format!(
            "/repos/{}/{}/milestones/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.milestone_number.to_string())
        );
        let answer: MilestoneAnswer =
            send::<_, ()>(self, http, token, Method::GET, &path, &[], None).await?;
        Ok(answer.into())
    }
}
