//! The result of scheduling a task.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scheduled {
    /// A new task was created.
    Created,
    /// A task with this name already exists, or ran recently. Nothing new was created, and the job
    /// is still scheduled exactly once.
    AlreadyScheduled,
}
