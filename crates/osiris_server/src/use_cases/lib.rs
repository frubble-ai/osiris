//! The helpers use cases share, by domain. Each takes the parts of the context it needs (the
//! database, the task scheduler, a setting), never the context, and never calls a use case. The
//! database's operations are here too, one per file in the domain they belong to.

pub mod auth_configs;
pub mod connected_accounts;
pub mod consent;
pub mod database;
pub mod events;
pub mod idempotency;
pub mod installation_tokens;
pub mod oauth;
pub mod refresh;
pub mod shared;
pub mod task_outcomes;
pub mod tasks;
