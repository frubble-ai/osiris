//! GitHub's actions, one use case each, grouped as GitHub's API is: each runs its provider call
//! through the connection, refused first when the connection lacks the action's scopes.

<<<<<<< HEAD
pub mod checks;
pub mod github_action_args;
pub mod installations;
<<<<<<< HEAD
pub mod issues;
=======
pub mod pull_requests;
>>>>>>> port/github-pull_requests
=======
pub mod activity;
pub mod github_action_args;
pub mod installations;
pub mod releases;
>>>>>>> port/github-releases_activity_search
pub mod repositories;
pub mod search;
pub mod users;
pub mod workflows;
