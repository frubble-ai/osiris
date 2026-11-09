//! `github.add_repository_collaborator`: an account added to a repository as a collaborator, or
//! its permission changed.

use crate::{
    ProviderError,
    github::{
        Github, path_segment::path_segment, request::send,
        transformers::collaborator_addition::RepositoryInvitationAnswer,
        types::GithubCollaboratorAddition,
    },
};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AddRepositoryCollaborator {
    pub owner: String,
    pub repo: String,
    /// The account's login.
    pub username: String,
    /// `pull`, `triage`, `push`, `maintain`, `admin`, or a custom repository role's name; GitHub
    /// grants `push` when left out.
    pub permission: Option<String>,
}

impl AddRepositoryCollaborator {
    /// The scopes it needs, as `OpenConnector` names them.
    pub const SCOPES: &[&str] = &["repo"];
}

#[derive(Serialize)]
struct Body<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    permission: Option<&'a str>,
}

impl Github {
    /// `PUT /repos/{owner}/{repo}/collaborators/{username}`. GitHub answers an invitation (a 201)
    /// when the account must accept it first, and nothing (a 204) when it was added at once.
    pub async fn add_repository_collaborator(
        &self,
        http: &reqwest::Client,
        token: &str,
        input: &AddRepositoryCollaborator,
    ) -> Result<GithubCollaboratorAddition, ProviderError> {
        let path = format!(
            "/repos/{}/{}/collaborators/{}",
            path_segment(&input.owner),
            path_segment(&input.repo),
            path_segment(&input.username)
        );
        let body = Body {
            permission: input.permission.as_deref(),
        };
        let answer: Option<RepositoryInvitationAnswer> =
            send(self, http, token, Method::PUT, &path, &[], Some(&body)).await?;
        Ok(answer.into())
    }
}
