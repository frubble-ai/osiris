//! The `trigger` table: each connection's triggers, the provider's id of what each subscribed to,
//! and what a trigger that checks on a schedule saw last. One operation per module.

pub mod create;
pub mod delete;
pub mod find;
pub mod list;
pub mod types;
pub mod update_state;

pub use types::{NewTrigger, TriggerState};
