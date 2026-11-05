//! The error returned when the scheduler cannot be reached or refuses a request.

#[derive(Debug, Clone, thiserror::Error)]
#[error("the task scheduler failed: {message}")]
pub struct TaskError {
    pub message: String,
}

impl TaskError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
