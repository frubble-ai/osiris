//! Slack's action routes, one per action, grouped as its API is.

pub mod conversations;
pub mod router;

pub use router::router;
