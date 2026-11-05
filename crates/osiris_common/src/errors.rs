//! Errors as error-stack reports: each layer has its own error type, and the chain of causes is
//! kept for the logs.

pub use error_stack::{Report, ResultExt};

/// A result whose error is a report of `E`, carrying every layer's context underneath.
pub type CustomResult<T, E> = Result<T, Report<E>>;
