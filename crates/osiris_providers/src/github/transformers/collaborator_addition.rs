//! The invitation GitHub answers when a collaborator must accept it first, as GitHub answers it.

use super::user::UserAnswer;
use crate::github::types::{GithubCollaboratorAddition, GithubRepositoryInvitation, GithubUser};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RepositoryInvitationAnswer {
    pub id: u64,
    #[serde(default)]
    pub permissions: String,
    #[serde(rename = "html_url", default)]
    pub url: String,
    pub invitee: Option<UserAnswer>,
    pub inviter: Option<UserAnswer>,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub expired: bool,
}

impl From<RepositoryInvitationAnswer> for GithubRepositoryInvitation {
    /// An invitation to collaborate on a repository, in our terms.
    fn from(answer: RepositoryInvitationAnswer) -> Self {
        Self {
            id: answer.id.to_string(),
            permissions: answer.permissions,
            url: answer.url,
            invitee: answer.invitee.map(GithubUser::from),
            inviter: answer.inviter.map(GithubUser::from),
            created_at: answer.created_at,
            expired: answer.expired,
        }
    }
}

impl From<Option<RepositoryInvitationAnswer>> for GithubCollaboratorAddition {
    /// GitHub answers an invitation (a 201) when the account must accept it first, and nothing
    /// (a 204) when the collaborator was added at once.
    fn from(answer: Option<RepositoryInvitationAnswer>) -> Self {
        Self {
            invited: answer.is_some(),
            invitation: answer.map(GithubRepositoryInvitation::from),
        }
    }
}
