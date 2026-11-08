//! GitHub's actions, one use case each, grouped as GitHub's API is: each runs its provider call
//! through the connection, refused first when the connection lacks the action's scopes.

pub mod checks;
pub mod github_action_args;
pub mod installations;
<<<<<<< HEAD
pub mod issues;
=======
pub mod pull_requests;
>>>>>>> port/github-pull_requests
pub mod repositories;
pub mod users;
pub mod workflows;
