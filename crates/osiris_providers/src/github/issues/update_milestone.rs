//! `github.update_milestone`: a milestone's title, state, description or due date changed.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send,
        transformers::milestone::MilestoneAnswer, types::GithubMilestone,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Only the fields given are changed.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct UpdateMilestone {
    pub owner: String,
    pub repo: String,
    pub milestone_number: u64,
    pub title: Option<String>,
    /// `open` or `closed`.
    pub state: Option<String>,
    pub description: Option<String>,
    /// When it is due, as an ISO 8601 timestamp.
    pub due_on: Option<String>,
}

impl UpdateMilestone {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

#[derive(Serialize)]
struct UpdateMilestoneBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    due_on: Option<&'a str>,
}

impl Github {
    /// `PATCH /repos/{owner}/{repo}/milestones/{milestone_number}`.
    pub async fn update_milestone(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &UpdateMilestone,
    ) -> Result<GithubMilestone, ProviderError> {
        let path = format!(
            "/repos/{}/{}/milestones/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.milestone_number.to_string())
        );
        let body = UpdateMilestoneBody {
            title: input.title.as_deref(),
            state: input.state.as_deref(),
            description: input.description.as_deref(),
            due_on: input.due_on.as_deref(),
        };
        let answer: MilestoneAnswer =
            send(self, http, token, Method::PATCH, &path, &[], Some(&body)).await?;
        Ok(answer.into())
    }
}
