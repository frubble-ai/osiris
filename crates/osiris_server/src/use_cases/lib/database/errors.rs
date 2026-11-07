//! Database failures as use case errors.

use error_stack::Report;
use osiris_domain::DomainError;

/// PostgreSQL's `character_not_in_repertoire`: a text value holding a NUL, which no text column
/// can store. Any string a client sends (an id in the path, a filter, a field) can hold one.
const NUL_IN_TEXT: &str = "22021";

/// A database error as a report: a value PostgreSQL can't store as text is the client's request
/// that isn't valid (400); anything else answers 500. The cause stays in the report for the logs.
pub fn db(error: sqlx::Error) -> Report<DomainError> {
    let refused_text = error
        .as_database_error()
        .and_then(sqlx::error::DatabaseError::code)
        .is_some_and(|code| code == NUL_IN_TEXT);
    if refused_text {
        return Report::new(error).change_context(DomainError::Invalid(
            "a value holds a NUL character, which osiris doesn't accept".to_owned(),
        ));
    }
    Report::new(error).change_context(DomainError::Internal)
}
