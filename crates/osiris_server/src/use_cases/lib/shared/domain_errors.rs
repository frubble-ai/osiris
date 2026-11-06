//! The errors use cases answer with: [`DomainError`]s, reported with their cause. The HTTP layer
//! maps each to its status and code (`http/error.rs`); anything else is a redacted 500.

use error_stack::Report;
use osiris_common::errors::CustomResult;
use osiris_domain::DomainError;
use osiris_interfaces::ProviderError;

pub type UseCaseResult<T> = CustomResult<T, DomainError>;

/// A provider's error as a domain report, keeping the provider's message in the report.
pub fn provider(error: ProviderError) -> Report<DomainError> {
    let domain = match &error {
        ProviderError::Unauthorized { message, .. } | ProviderError::Refused { message, .. } => {
            DomainError::ProviderRefused(message.clone())
        }
        ProviderError::Unavailable { message, .. } => {
            DomainError::ProviderUnavailable(message.clone())
        }
    };
    Report::new(error).change_context(domain)
}

/// Anything a use case can't go on with, kept in the report as its cause.
pub fn internal(error: impl std::error::Error + Send + Sync + 'static) -> Report<DomainError> {
    Report::new(error).change_context(DomainError::Internal)
}
