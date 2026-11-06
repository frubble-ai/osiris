//! The `task_outcome` table: how each task run ended. One operation per module.

pub mod record;

pub use record::Recorded;

#[cfg(test)]
mod tests;
