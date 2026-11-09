//! The registered provider of one type, for that provider's own actions.

use super::domain_errors::UseCaseResult;
use error_stack::ResultExt;
use osiris_domain::DomainError;
use osiris_providers::{Provider, Providers};

/// The provider of type `P`. Every provider is registered at boot, so one missing is a bug.
pub fn provider_of<P: Provider + 'static>(providers: &Providers) -> UseCaseResult<&P> {
    providers
        .typed::<P>()
        .ok_or(DomainError::Internal)
        .attach_with(|| format!("no provider of type {}", std::any::type_name::<P>()))
}
