//! GitHub's actions, one use case each, grouped as GitHub's API is: each runs its provider call
//! through the connection, refused first when the connection lacks the action's scopes.

pub mod activity;
pub mod github_action_args;
pub mod installations;
pub mod releases;
pub mod repositories;
pub mod search;
pub mod users;
