//! A valid access token for a complete connection to the action's provider. The token is
//! refreshed here only if it is about to expire: a safety net, since its refresh task normally
//! refreshes it in time. A connection disabled only because its refresh could not be scheduled is
//! restored here, if the scheduler is reachable again.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        get_connected_account::GetConnectedAccountArgs, refresh_connection::RefreshConnectionArgs,
    },
    lib::refresh::{When, restore::restore},
};
use osiris_common::secret::Secret;
use osiris_domain::{ConnectionStatus, DomainError};
use time::OffsetDateTime;

pub struct ConnectionTokenArgs<'a> {
    pub id: &'a str,
    pub provider: &'a str,
    pub action: &'a str,
}

impl UseCases {
    pub async fn connection_token(&self, args: ConnectionTokenArgs<'_>) -> UseCaseResult<Secret> {
        self.run("connection_token", async {
            let ctx = self.context();
            let ConnectionTokenArgs { id, provider, action } = args;
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
                self.refresh_connection(RefreshConnectionArgs {
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
            account
                .tokens
                .map(|tokens| tokens.access_token)
                .ok_or_else(|| DomainError::NotActive("without tokens").into())
        })
        .await
    }
}
