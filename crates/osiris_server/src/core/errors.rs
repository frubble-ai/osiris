//! The core's results: domain errors, reported with their cause.

use error_stack::Report;
use osiris_common::errors::CustomResult;
use osiris_domain::DomainError;
use osiris_interfaces::ProviderError;

pub type CoreResult<T> = CustomResult<T, DomainError>;

/// A provider's error as a domain report, keeping the provider's message.
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

/// Anything the core can't go on with, kept in the report as its cause.
pub fn internal(error: impl std::error::Error + Send + Sync + 'static) -> Report<DomainError> {
    Report::new(error).change_context(DomainError::Internal)
}
