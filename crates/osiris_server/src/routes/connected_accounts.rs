//! `/v1/connected-accounts`: connecting users, and the connections osiris holds. API key.

pub mod create;
pub mod delete;
pub mod list;
pub mod refresh;
pub mod retrieve;
pub mod router;

pub use router::router;
