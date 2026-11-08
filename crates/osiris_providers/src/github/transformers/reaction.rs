//! A reaction to an issue or a comment, as GitHub answers it.

use super::user::UserAnswer;
use crate::github::types::{GithubReaction, GithubUser};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ReactionAnswer {
    pub id: u64,
    #[serde(default)]
    pub content: String,
    pub user: Option<UserAnswer>,
    #[serde(default)]
    pub created_at: String,
}

impl From<ReactionAnswer> for GithubReaction {
    /// A reaction to an issue or a comment, in our terms.
    fn from(answer: ReactionAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            content: answer.content,
            user: answer.user.map(GithubUser::from),
            created_at: answer.created_at,
        }
    }
}
