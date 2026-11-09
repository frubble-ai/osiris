//! A comment on a commit, as GitHub answers it.

use super::{reactions::ReactionsAnswer, user::UserAnswer};
use crate::github::types::{GithubCommitComment, GithubReactions, GithubUser};
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
    #[serde(default)]
    pub commit_id: String,
    pub line: Option<i64>,
    #[serde(default)]
    pub author_association: String,
    pub reactions: Option<ReactionsAnswer>,
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
            commit_id: answer.commit_id,
            line: answer.line,
            author_association: answer.author_association,
            reactions: answer.reactions.map(GithubReactions::from),
        }
    }
}
