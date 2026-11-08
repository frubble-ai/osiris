//! GitHub's action routes, one per action, grouped as GitHub's API is.

pub mod checks;
pub mod installations;
pub mod repositories;
pub mod router;
pub mod users;
pub mod workflows;

pub use router::router;
