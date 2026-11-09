//! A milestone, as GitHub answers it.

use super::user::UserAnswer;
use crate::github::types::{GithubMilestone, GithubUser};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MilestoneAnswer {
    pub id: u64,
    #[serde(default)]
    pub number: i64,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub state: String,
    pub description: Option<String>,
    pub due_on: Option<String>,
    #[serde(default)]
    pub open_issues: i64,
    #[serde(default)]
    pub closed_issues: i64,
    #[serde(rename = "html_url", default)]
    pub url: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub closed_at: Option<String>,
    pub creator: Option<UserAnswer>,
}

impl From<MilestoneAnswer> for GithubMilestone {
    /// A milestone, in our terms.
    fn from(answer: MilestoneAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            number: answer.number,
            title: answer.title,
            state: answer.state,
            description: answer.description,
            due_on: answer.due_on,
            open_issues: answer.open_issues,
            closed_issues: answer.closed_issues,
            url: answer.url,
            created_at: answer.created_at,
            updated_at: answer.updated_at,
            closed_at: answer.closed_at,
            creator: answer.creator.map(GithubUser::from),
        }
    }
}
