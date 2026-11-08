//! Scheduling and cancelling tasks through the task scheduler, and recording how a task run ended.

pub mod cancel_quietly;
pub mod next_trigger_check;
pub mod record_outcome;
pub mod schedule;
pub mod scheduling;

pub use scheduling::Scheduling;

#[cfg(test)]
mod tests;
