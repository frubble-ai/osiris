//! Slack's actions, one use case each, grouped as its API is: each runs its provider call
//! through the connection, refused first when the connection lacks the action's scopes.

pub mod conversations;
