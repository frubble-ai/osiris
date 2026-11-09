//! `/v1/triggers`: a connection's triggers, created at and removed from its provider. API key.

pub mod create;
pub mod delete;
pub mod list;
pub mod retrieve;
pub mod router;

pub use router::router;
