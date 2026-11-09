//! A comment on an issue or a pull request, as GitHub answers it.

use super::{reactions::ReactionsAnswer, user::UserAnswer};
use crate::github::types::{GithubIssueComment, GithubReactions, GithubUser};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct IssueCommentAnswer {
    pub id: u64,
    #[serde(rename = "html_url", default)]
    pub url: String,
    #[serde(default)]
    pub body: String,
    pub user: Option<UserAnswer>,
    pub reactions: Option<ReactionsAnswer>,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub author_association: String,
}

impl From<IssueCommentAnswer> for GithubIssueComment {
    /// A comment on an issue or a pull request, in our terms.
    fn from(answer: IssueCommentAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            url: answer.url,
            body: answer.body,
            user: answer.user.map(GithubUser::from),
            reactions: answer.reactions.map(GithubReactions::from),
            created_at: answer.created_at,
            updated_at: answer.updated_at,
            author_association: answer.author_association,
        }
    }
}
