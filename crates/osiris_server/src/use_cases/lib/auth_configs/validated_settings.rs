//! An app's settings as its provider accepts them: `settings` as an object, and `secret_settings`
//! read from their JSON text, checked by the provider before anything is stored.

use crate::use_cases::UseCaseResult;
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_domain::DomainError;
use osiris_interfaces::Provider;
use serde_json::Value;

/// `settings` and `secret_settings` if `provider` accepts them; refused as invalid otherwise.
pub fn validated_settings(
    provider: &dyn Provider,
    settings: &Value,
    secret_settings: Option<&Secret>,
) -> UseCaseResult<()> {
    let secret = secret_settings
        .map(|text| serde_json::from_str::<Value>(text.expose_secret()))
        .transpose()
        .map_err(|_| DomainError::Invalid("secret_settings must be a JSON object".into()))?;
    provider
        .validate_settings(settings, secret.as_ref())
        .map_err(|reason| DomainError::Invalid(reason).into())
}
