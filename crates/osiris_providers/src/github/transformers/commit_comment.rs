//! A comment on a commit, as GitHub answers it.

use super::user::UserAnswer;
use crate::github::types::{GithubCommitComment, GithubUser};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CommitCommentAnswer {
    pub id: u64,
    #[serde(default)]
    pub body: String,
    #[serde(rename = "html_url", default)]
    pub url: String,
    pub path: Option<String>,
    pub position: Option<i64>,
    pub user: Option<UserAnswer>,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

impl From<CommitCommentAnswer> for GithubCommitComment {
    /// A comment on a commit, in our terms.
    fn from(answer: CommitCommentAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            body: answer.body,
            url: answer.url,
            path: answer.path,
            position: answer.position,
            user: answer.user.map(GithubUser::from),
            created_at: answer.created_at,
            updated_at: answer.updated_at,
        }
    }
}
