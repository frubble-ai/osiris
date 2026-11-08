//! `github.create_milestone`: a new milestone in a repository.

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

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateMilestone {
    pub owner: String,
    pub repo: String,
    pub title: String,
    /// `open` (GitHub's default) or `closed`.
    pub state: Option<String>,
    pub description: Option<String>,
    /// When it is due, as an ISO 8601 timestamp.
    pub due_on: Option<String>,
}

impl CreateMilestone {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

#[derive(Serialize)]
struct CreateMilestoneBody<'a> {
    title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    due_on: Option<&'a str>,
}

impl Github {
    /// `POST /repos/{owner}/{repo}/milestones`.
    pub async fn create_milestone(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &CreateMilestone,
    ) -> Result<GithubMilestone, ProviderError> {
        let path = format!(
            "/repos/{}/{}/milestones",
            path_segment(&input.owner),
            path_segment(&input.repo)
        );
        let body = CreateMilestoneBody {
            title: &input.title,
            state: input.state.as_deref(),
            description: input.description.as_deref(),
            due_on: input.due_on.as_deref(),
        };
        let answer: MilestoneAnswer =
            send(self, http, token, Method::POST, &path, &[], Some(&body)).await?;
        Ok(answer.into())
    }
}
