//! GitHub's action routes, one per action, grouped as GitHub's API is.

pub mod activity;
pub mod installations;
pub mod releases;
pub mod repositories;
pub mod router;
pub mod search;
pub mod users;

pub use router::router;
