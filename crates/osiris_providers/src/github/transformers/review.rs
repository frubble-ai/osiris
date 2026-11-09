//! A review of a pull request, as GitHub answers it.

use super::user::UserAnswer;
use crate::github::types::{GithubReview, GithubUser};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ReviewAnswer {
    pub id: u64,
    pub user: Option<UserAnswer>,
    pub body: Option<String>,
    #[serde(default)]
    pub state: String,
    #[serde(rename = "html_url", default)]
    pub url: String,
    pub commit_id: Option<String>,
    pub submitted_at: Option<String>,
    #[serde(default)]
    pub author_association: String,
}

impl From<ReviewAnswer> for GithubReview {
    /// A review of a pull request, in our terms.
    fn from(answer: ReviewAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            user: answer.user.map(GithubUser::from),
            body: answer.body,
            state: answer.state,
            url: answer.url,
            commit_id: answer.commit_id,
            submitted_at: answer.submitted_at,
            author_association: answer.author_association,
        }
    }
}
