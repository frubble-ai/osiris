//! A collaborator added to a repository, and the invitation GitHub sends when the account must
//! accept first.

use super::GithubUser;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// An invitation to collaborate on a repository, waiting for the invited account to accept it.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct GithubRepositoryInvitation {
    pub id: String,
    /// `read`, `triage`, `write`, `maintain` or `admin` today, as GitHub names it. An open string.
    pub permissions: String,
    /// Its page on GitHub.
    pub url: String,
    pub invitee: Option<GithubUser>,
    pub inviter: Option<GithubUser>,
    pub created_at: String,
    pub expired: bool,
}

/// A collaborator added to a repository: at once when `invited` is false (an organization member,
/// or a permission changed), or through `invitation` when the account must accept it first.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct GithubCollaboratorAddition {
    pub invited: bool,
    pub invitation: Option<GithubRepositoryInvitation>,
}
