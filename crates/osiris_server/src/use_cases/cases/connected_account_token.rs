//! A valid access token for a complete connection to the action's provider. The token is
//! refreshed here only if it is about to expire: a safety net, since its refresh task normally
//! refreshes it in time. A connection disabled only because its refresh could not be scheduled is
//! restored here, if the scheduler is reachable again. A connection granted scopes that don't cover
//! the action's is refused here, before the provider is called.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        get_connected_account::GetConnectedAccountArgs,
        refresh_connected_account::RefreshConnectedAccountArgs,
    },
    lib::{
        refresh::{When, restore::restore},
        shared::missing_scopes::missing_scopes,
    },
};
use osiris_common::secret::Secret;
use osiris_domain::{ConnectionStatus, DomainError};
use time::OffsetDateTime;

pub struct ConnectedAccountTokenArgs<'a> {
    pub id: &'a str,
    pub provider: &'a str,
    pub action: &'a str,
    /// The scopes the action needs; a connection granted scopes that don't cover them is refused
    /// before the provider is called (`missing_scopes`).
    pub required_scopes: &'a [&'a str],
}

impl UseCases {
    pub async fn connected_account_token(
        &self,
        args: ConnectedAccountTokenArgs<'_>,
    ) -> UseCaseResult<Secret> {
        self.run("connected_account_token", async {
            let ctx = self.context();
            let ConnectedAccountTokenArgs {
                id,
                provider,
                action,
                required_scopes,
            } = args;
            let account = self.get_connected_account(GetConnectedAccountArgs { id }).await?;
            if account.provider != provider {
                return Err(DomainError::Invalid(format!(
                    "{action} runs on a {provider} connection; {id} is {}",
                    account.provider
                ))
                .into());
            }
            // Only a token about to expire takes the refresh lock; a valid one is used as it is.
            let due = account.tokens.as_ref().is_some_and(|tokens| {
                tokens.needs_refresh(
                    OffsetDateTime::now_utc(),
                    ctx.config.settings.refresh.margin(),
                )
            });
            let account = if due {
                self.refresh_connected_account(RefreshConnectedAccountArgs {
                    id,
                    when: When::IfDue,
                    running: None,
                })
                .await?
                .into_account()
                .ok_or(DomainError::Internal)?
            } else {
                account
            };
            let account = match restore(
                &ctx.db,
                ctx.scheduling(),
                &ctx.config.settings.refresh,
                &account,
            )
            .await
            {
                Ok(Some(restored)) => restored,
                Ok(None) => account,
                Err(report) => {
                    tracing::warn!(error = ?report, connected_account_id = id, "the refresh still cannot be scheduled");
                    account
                }
            };
            if account.status != ConnectionStatus::Active {
                return Err(DomainError::NotActive(account.status.as_str()).into());
            }
            if !account.is_complete() {
                return Err(DomainError::NotActive("disabled").into());
            }
            if let Some(implementation) = ctx.providers.get(provider) {
                let granted = account.tokens.as_ref().and_then(|tokens| tokens.scope.as_deref());
                let missing = missing_scopes(implementation, granted, required_scopes);
                if !missing.is_empty() {
                    return Err(DomainError::MissingScopes(format!(
                        "{action} needs {}; reconnect granting them",
                        missing.join(", ")
                    ))
                    .into());
                }
            }
            account
                .tokens
                .map(|tokens| tokens.access_token)
                .ok_or_else(|| DomainError::NotActive("without tokens").into())
        })
        .await
    }
}
