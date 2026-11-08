//! A token for an installation connection: the one this process holds, or a new one minted with
//! the app's credentials and kept until shortly before it expires.

use super::InstallationTokenCache;
use super::mint_refused::MintRefused;
use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::shared::domain_errors::provider;
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_domain::{AuthConfig, DomainError};
use osiris_interfaces::{Provider, ProviderError};
use serde_json::Value;
use time::{Duration, OffsetDateTime, format_description::well_known::Rfc3339};

/// The token for `installation_id` through `config`'s app, from `cache` when it holds one good
/// past `margin`, minted with `implementation` otherwise.
pub async fn installation_token(
    implementation: &dyn Provider,
    http: &reqwest::Client,
    cache: &InstallationTokenCache,
    margin: Duration,
    config: &AuthConfig,
    installation_id: &str,
) -> UseCaseResult<Secret> {
    let now = OffsetDateTime::now_utc();
    if let Some(token) = cache.get(&config.id, installation_id, now, margin) {
        return Ok(token);
    }
    let secret_settings = config
        .secret_settings
        .as_ref()
        .map(|text| serde_json::from_str::<Value>(text.expose_secret()))
        .transpose()
        .map_err(|_| DomainError::Internal)?
        .ok_or_else(|| {
            DomainError::Invalid(
                "the auth config holds no app credentials (secret_settings) to act as an installation".into(),
            )
        })?;
    let minted = implementation
        .installation_token(http, &config.settings, &secret_settings, installation_id)
        .await
        .map_err(|error| {
            let refused = matches!(
                error,
                ProviderError::Refused { .. } | ProviderError::Unauthorized { .. }
            );
            let report = provider(error);
            if refused {
                report.attach(MintRefused)
            } else {
                report
            }
        })?;
    // A token whose expiry can't be read is used once and not kept.
    let token = minted.token;
    if let Ok(expires_at) = OffsetDateTime::parse(&minted.expires_at, &Rfc3339) {
        cache.put(&config.id, installation_id, token.clone(), expires_at);
    }
    Ok(token)
}
