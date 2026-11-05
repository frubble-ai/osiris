//! The helpers use cases share, by domain. Each takes the parts of the context it needs (the store,
//! the task scheduler, a setting), never the context, and never calls a use case.

pub mod consent;
pub mod events;
pub mod oauth;
pub mod refresh;
pub mod shared;
pub mod tasks;
