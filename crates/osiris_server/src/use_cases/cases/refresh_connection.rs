//! Refreshes one connection.
//!
//! When the refresh comes from a task, the connection must still be active and on the task's token
//! generation; otherwise the task is out of date and nothing happens.
//!
//! On success, the next refresh task is scheduled first, and then the new tokens and that task's
//! name are saved in one write. If the next task cannot be scheduled, the new tokens are still
//! saved (a provider may already have invalidated the old refresh token), the connection is
//! disabled, and an error is returned. The task's retry, or the next action, then schedules the
//! refresh and makes the connection active again (see `restore`). An active connection never lacks
//! its scheduled refresh.
//!
//! What happens on a failure depends on the provider's answer. A hard refusal (`invalid_grant`, a
//! revoked grant, any other 4xx) means the grant is dead: the connection is marked as needing
//! reconnection, its event is published, and nothing is scheduled. An outage or a timeout
//! (network errors, 408, 429, 5xx) keeps the connection active: the failure is counted and an
//! error is returned so the refresh is tried again.

use crate::use_cases::{
    UseCaseResult, UseCases,
    lib::{
        oauth::{new_tokens::new_tokens, oauth_client::oauth_client},
        refresh::{
            Refresh, RunningTask, When,
            check_task::{Checked, check_task},
            end_refused::end_refused,
            save_refreshed::{SaveParts, save_refreshed},
        },
        shared::{domain_errors::provider as provider_error, redirect_uri::redirect_uri},
    },
};
use osiris_common::secret::ExposeSecret;
use osiris_domain::{ConnectionStatus, DomainError};
use osiris_interfaces::{ProviderError, oauth};

use time::OffsetDateTime;

pub struct RefreshConnectionArgs<'a> {
    pub id: &'a str,
    pub when: When,
    /// The refresh task asking, when a task asks.
    pub running: Option<RunningTask<'a>>,
}

impl UseCases {
    pub async fn refresh_connection(
        &self,
        args: RefreshConnectionArgs<'_>,
    ) -> UseCaseResult<Refresh> {
        self.run("refresh_connection", async {
            let ctx = self.context();
            let RefreshConnectionArgs { id, when, running } = args;
            let settings = &ctx.config.settings;
            // Everything the refresh needs to read is read before the row is locked, so a refresh
            // holding a lock never waits for a second database connection.
            let unlocked = match ctx.db.connected_account(id).await {
                Ok(account) => account,
                Err(report)
                    if running.is_some()
                        && matches!(report.current_context(), DomainError::NotFound(_)) =>
                {
                    return Ok(Refresh::Stale);
                }
                Err(report) => return Err(report),
            };
            let config = ctx.db.auth_config(&unlocked.auth_config_id).await?;
            let lock = ctx.db.lock_for_refresh(id).await?;
            let lock = match check_task(
                &ctx.db,
                ctx.scheduling(),
                &settings.refresh,
                lock,
                running,
            )
            .await?
            {
                Checked::Go(lock) => *lock,
                Checked::Done(refresh) => return Ok(*refresh),
            };
            let account = lock.account.clone();
            let due = account.tokens.as_ref().is_some_and(|tokens| {
                when == When::Now
                    || tokens.needs_refresh(OffsetDateTime::now_utc(), settings.refresh.margin())
            });
            let refresh_token = account
                .tokens
                .as_ref()
                .and_then(|tokens| tokens.refresh_token.clone());
            let (true, ConnectionStatus::Active, Some(refresh_token)) =
                (due, account.status, refresh_token)
            else {
                lock.release().await?;
                return Ok(Refresh::Unchanged(account));
            };
            let provider = ctx
                .providers
                .get(&config.provider)
                .ok_or(DomainError::Internal)?;
            let redirect_uri = redirect_uri(&ctx.config.public_url);
            let answer = oauth::refresh(
                provider.id(),
                &ctx.http,
                provider.oauth(),
                &oauth_client(&config, &redirect_uri),
                refresh_token.expose_secret(),
            )
            .await;
            let previous_task = account.refresh_task.clone();
            match answer {
                Ok(fresh) => {
                    save_refreshed(
                        SaveParts {
                            tasks: ctx.tasks.clone(),
                            schedule_timeout: settings.tasks.schedule_timeout(),
                            refresh: settings.refresh.clone(),
                        },
                        lock,
                        new_tokens(fresh),
                        running,
                    )
                    .await
                }
                Err(
                    ProviderError::Refused { message, .. }
                    | ProviderError::Unauthorized { message, .. },
                ) => {
                    tracing::warn!(connected_account_id = id, %message, "the provider permanently refused the refresh; the user must reconnect");
                    end_refused(
                        &ctx.db,
                        &ctx.messages,
                        ctx.scheduling(),
                        lock,
                        previous_task.as_deref(),
                        running,
                    )
                    .await
                }
                Err(error @ ProviderError::Unavailable { .. }) => {
                    tracing::warn!(connected_account_id = id, %error, "the provider failed the refresh for now; it will be tried again");
                    lock.failed("the provider could not be reached; retrying")
                        .await?;
                    Err(provider_error(error))
                }
            }
        })
        .await
    }
}
