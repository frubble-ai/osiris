//! An installation connection's token: minted from its app's credentials, or the one this process
//! holds reused. Nothing is stored.

use super::installation_token::installation_token;
use crate::use_cases::{UseCaseContext, UseCaseResult};
use osiris_common::secret::Secret;
use osiris_domain::{ConnectedAccount, ConnectionStatus, DomainError};

/// `account`'s token, an installation connection to `provider` that must be active.
pub async fn connection_token(
    ctx: &UseCaseContext,
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
    let config = ctx.db.auth_config(&account.auth_config_id).await?;
    let implementation = ctx.providers.get(provider).ok_or(DomainError::Internal)?;
    installation_token(
        implementation,
        &ctx.http,
        &ctx.installation_tokens,
        ctx.config.settings.providers.installation_tokens.margin(),
        &config,
        installation_id,
    )
    .await
}
