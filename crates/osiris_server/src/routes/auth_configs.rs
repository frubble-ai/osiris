//! `/v1/auth-configs`: the providers' apps. Admin key.

pub mod create;
pub mod delete;
pub mod list;
pub mod retrieve;
pub mod router;
pub mod update;

pub use router::router;
