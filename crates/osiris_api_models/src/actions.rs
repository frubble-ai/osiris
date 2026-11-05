//! `/v1/connected-accounts/{id}/actions/…`: each action's answer, typed.

pub use osiris_providers::{
    github::types::{
        GithubAccountType, GithubInstallation, GithubRepository, GithubRepositorySelection,
    },
    linear::types::LinearTeam,
    slack::types::SlackChannel,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// An action's answer: what it read, as `data`.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ActionResult<T> {
    pub data: T,
}
