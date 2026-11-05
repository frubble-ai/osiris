//! Scheduled tasks: scheduling them through the task scheduler, and running each one when the
//! scheduler sends it to the worker. Before doing anything, a task checks that its job is still
//! wanted. The outcome of every run is recorded.

pub mod cancel;
pub mod expire_consent;
pub mod publish_event;
pub mod record;
pub mod run_refresh;
pub mod run_task;
pub mod schedule;
