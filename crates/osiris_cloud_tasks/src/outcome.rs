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
mod tests {
    use super::*;
    use google_cloud_gax::error::rpc::Status;

    fn refused_with(code: Code) -> Error {
        Error::service(Status::default().set_code(code).set_message("refused"))
    }

    #[test]
    fn counts_a_new_task_created_and_an_existing_one_as_scheduled_once() {
        assert_eq!(created(Ok(())).unwrap(), Scheduled::Created);
        assert_eq!(
            created(Err(refused_with(Code::AlreadyExists))).unwrap(),
            Scheduled::AlreadyScheduled
        );
    }

    #[test]
    fn fails_any_other_refusal_to_create() {
        for code in [
            Code::PermissionDenied,
            Code::Unavailable,
            Code::InvalidArgument,
        ] {
            assert!(created(Err(refused_with(code))).is_err(), "{code:?}");
        }
    }

    #[test]
    fn counts_a_task_already_gone_as_cancelled() {
        assert!(cancelled(Ok(())).is_ok());
        assert!(cancelled(Err(refused_with(Code::NotFound))).is_ok());
    }

    #[test]
    fn fails_any_other_refusal_to_cancel() {
        for code in [Code::PermissionDenied, Code::Unavailable] {
            assert!(cancelled(Err(refused_with(code))).is_err(), "{code:?}");
        }
    }
}
