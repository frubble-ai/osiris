//! An installation connection's token: the one this process holds, looked up first, or one minted
//! from its app's credentials, which only then are read and decrypted. Nothing is stored.

use super::{InstallationTokenCache, installation_token::installation_token};
use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::Db;
use osiris_common::secret::Secret;
use osiris_domain::{ConnectedAccount, ConnectionStatus, DomainError};
use osiris_providers::Providers;
use time::{Duration, OffsetDateTime};

/// Where an installation connection's token comes from: the parts of the use cases' context it
/// reads.
pub struct TokenSources<'a> {
    pub db: &'a Db,
    pub providers: &'a Providers,
    pub http: &'a reqwest::Client,
    pub cache: &'a InstallationTokenCache,
    /// A cached token is used only when it is good for longer than this.
    pub margin: Duration,
}

/// `account`'s token, an installation connection to `provider` that must be active.
pub async fn connection_token(
    sources: TokenSources<'_>,
    account: &ConnectedAccount,
    provider: &str,
) -> UseCaseResult<Secret> {
    if account.status != ConnectionStatus::Active {
        return Err(DomainError::NotActive(account.status.as_str()).into());
    }
    let installation_id = account
        .installation_id
        .as_deref()
        .ok_or(DomainError::Internal)?;
    let now = OffsetDateTime::now_utc();
    if let Some(token) = sources.cache.get(
        &account.auth_config_id,
        installation_id,
        now,
        sources.margin,
    ) {
        return Ok(token);
    }
    let config = sources.db.auth_config(&account.auth_config_id).await?;
    let implementation = sources
        .providers
        .get(provider)
        .ok_or(DomainError::Internal)?;
    installation_token(
        implementation,
        sources.http,
        sources.cache,
        sources.margin,
        &config,
        installation_id,
    )
    .await
}
