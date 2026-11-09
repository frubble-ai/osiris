//! GitHub's actions, one use case each, grouped as its API is: each runs its provider call
//! through the connection, refused first when the connection lacks the action's scopes.

pub mod activity;
pub mod checks;
pub mod installations;
pub mod issues;
pub mod pull_requests;
pub mod releases;
pub mod repositories;
pub mod search;
pub mod users;
pub mod workflows;

#[cfg(test)]
mod tests;
