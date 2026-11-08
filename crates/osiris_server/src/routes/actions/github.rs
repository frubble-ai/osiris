//! GitHub's action routes, one per action, grouped as GitHub's API is.

<<<<<<< HEAD
pub mod checks;
pub mod installations;
<<<<<<< HEAD
pub mod issues;
=======
pub mod pull_requests;
>>>>>>> port/github-pull_requests
=======
pub mod activity;
pub mod installations;
pub mod releases;
>>>>>>> port/github-releases_activity_search
pub mod repositories;
pub mod router;
pub mod search;
pub mod users;
pub mod workflows;

pub use router::router;
