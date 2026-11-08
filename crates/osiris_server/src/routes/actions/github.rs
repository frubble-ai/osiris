//! GitHub's action routes, one per action, grouped as GitHub's API is.

pub mod installations;
pub mod repositories;
pub mod router;
pub mod users;

pub use router::router;
