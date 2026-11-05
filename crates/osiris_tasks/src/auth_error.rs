//! Why a task's caller was refused.

#[derive(Debug, Clone, thiserror::Error)]
#[error("the task's caller was refused: {reason}")]
pub struct AuthError {
    pub reason: String,
}

impl AuthError {
    pub fn new(reason: impl Into<String>) -> Self {
        Self {
            reason: reason.into(),
        }
    }
}
