//! What storage can fail with.

use osiris_common::errors::ErrorSwitch;
use osiris_domain::DomainError;

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("{0} not found")]
    NotFound(&'static str),
    #[error("{0}")]
    Conflict(String),
    #[error("a secret could not be encrypted")]
    Encrypt,
    #[error("a stored secret could not be decrypted")]
    Decrypt,
    #[error("the database failed")]
    Database,
}

impl ErrorSwitch<DomainError> for StorageError {
    fn switch(&self) -> DomainError {
        match self {
            Self::NotFound(what) => DomainError::NotFound(what),
            Self::Conflict(message) => DomainError::Conflict(message.clone()),
            Self::Encrypt | Self::Decrypt | Self::Database => DomainError::Internal,
        }
    }
}

pub type StorageResult<T> = osiris_common::errors::CustomResult<T, StorageError>;

/// A database error as a storage report.
pub fn db(error: sqlx::Error) -> error_stack::Report<StorageError> {
    error_stack::Report::new(error).change_context(StorageError::Database)
}
