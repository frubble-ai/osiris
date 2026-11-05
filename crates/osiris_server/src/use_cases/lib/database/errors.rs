//! Database failures as use case errors.

use error_stack::Report;
use osiris_domain::DomainError;

/// A database error as a report that answers 500; the cause stays in the report for the logs.
pub fn db(error: sqlx::Error) -> Report<DomainError> {
    Report::new(error).change_context(DomainError::Internal)
}
