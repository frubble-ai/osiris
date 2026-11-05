//! `/v1/connected-accounts/{id}/actions/…`: each action's answer, typed, one module per provider.

pub mod action_result;
pub mod github;
pub mod linear;
pub mod slack;

pub use action_result::ActionResult;
pub use github::{
    GithubAccountType, GithubInstallation, GithubRepository, GithubRepositorySelection,
};
pub use linear::LinearTeam;
pub use slack::SlackChannel;
