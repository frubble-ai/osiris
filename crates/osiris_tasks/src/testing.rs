//! For tests: the contract every `TaskScheduler` is held to, and generated tasks.

#![allow(
    clippy::expect_used,
    clippy::missing_panics_doc,
    reason = "a contract's failure is the test failing"
)]

pub mod generate_task;
pub mod scheduler_contract;

pub use scheduler_contract::{SchedulerHarness, task_scheduler_contract};
