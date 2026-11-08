//! GitHub's action routes, one per action, grouped as GitHub's API is.

pub mod activity;
pub mod checks;
pub mod installations;
pub mod issues;
pub mod pull_requests;
pub mod releases;
pub mod repositories;
pub mod router;
pub mod search;
pub mod users;
pub mod workflows;

pub use router::router;
