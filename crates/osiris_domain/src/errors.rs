//! What use cases answer with when they cannot do what was asked. The wire layer maps each to a status and
//! an `OS_xx` code; the report underneath keeps the cause for the logs.

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("{0} not found")]
    NotFound(&'static str),
    #[error("{0}")]
    Invalid(String),
    #[error("{0}")]
    Conflict(String),
    /// The provider refused: the user may have to reconnect.
    #[error("the provider refused: {0}")]
    ProviderRefused(String),
    /// The provider could not be reached or answered something unreadable; retrying may help.
    #[error("the provider failed: {0}")]
    ProviderUnavailable(String),
    #[error("the connection is {0}: reconnect it")]
    NotActive(&'static str),
    /// The connection wasn't granted a scope the action needs; reconnecting with it fixes that.
    #[error("the connection lacks the scopes this action needs: {0}")]
    MissingScopes(String),
    #[error("an internal error")]
    Internal,
}
