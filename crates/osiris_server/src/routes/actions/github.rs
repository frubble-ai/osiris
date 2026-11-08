//! GitHub's action routes, one per action, grouped as GitHub's API is.

pub mod installations;
<<<<<<< HEAD
pub mod issues;
=======
pub mod pull_requests;
>>>>>>> port/github-pull_requests
pub mod repositories;
pub mod router;
pub mod users;

pub use router::router;
