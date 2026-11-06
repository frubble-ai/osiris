//! What Cloud Tasks' answers mean for the scheduler: a task that already exists is scheduled once,
//! not twice, and a task that is already gone is as good as cancelled. Anything else is a failure.

use google_cloud_gax::error::{Error, rpc::Code};
use osiris_tasks::{Scheduled, TaskError};

/// The result of creating a task.
pub fn created(result: Result<(), Error>) -> Result<Scheduled, TaskError> {
    match result {
        Ok(()) => Ok(Scheduled::Created),
        Err(error) if has_code(&error, Code::AlreadyExists) => Ok(Scheduled::AlreadyScheduled),
        Err(error) => Err(TaskError::new(error.to_string())),
    }
}

/// The result of deleting a task.
pub fn cancelled(result: Result<(), Error>) -> Result<(), TaskError> {
    match result {
        Ok(()) => Ok(()),
        Err(error) if has_code(&error, Code::NotFound) => Ok(()),
        Err(error) => Err(TaskError::new(error.to_string())),
    }
}

fn has_code(error: &Error, code: Code) -> bool {
    error.status().is_some_and(|status| status.code == code)
}

#[cfg(test)]
mod tests;
