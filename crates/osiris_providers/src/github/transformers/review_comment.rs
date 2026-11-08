//! A comment on a line of a pull request's diff, as GitHub answers it.

use super::user::UserAnswer;
use crate::github::types::{GithubReviewComment, GithubUser};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ReviewCommentAnswer {
    pub id: u64,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub body: String,
    pub user: Option<UserAnswer>,
    #[serde(default)]
    pub commit_id: String,
    #[serde(default)]
    pub original_commit_id: String,
    #[serde(default)]
    pub diff_hunk: String,
    #[serde(rename = "html_url", default)]
    pub url: String,
    pub line: Option<i64>,
    pub start_line: Option<i64>,
    pub side: Option<String>,
    pub start_side: Option<String>,
    pub in_reply_to_id: Option<u64>,
}

impl From<ReviewCommentAnswer> for GithubReviewComment {
    /// A comment on a line of a pull request's diff, in our terms.
    fn from(answer: ReviewCommentAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            path: answer.path,
            body: answer.body,
            user: answer.user.map(GithubUser::from),
            commit_id: answer.commit_id,
            original_commit_id: answer.original_commit_id,
            diff_hunk: answer.diff_hunk,
            url: answer.url,
            line: answer.line,
            start_line: answer.start_line,
            side: answer.side,
            start_side: answer.start_side,
            in_reply_to_id: answer.in_reply_to_id.map(|id| id.to_string()),
        }
    }
}
