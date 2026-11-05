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

use super::{Refresh, RunningTask, When, new_tokens::new_tokens};
use crate::core::{
    Core, CoreResult, errors::provider as provider_error, oauth_client::oauth_client,
};
use osiris_common::{errors::ReportSwitchExt, secret::ExposeSecret};
use osiris_domain::{ConnectionStatus, DomainError};
use osiris_interfaces::{ProviderError, oauth};
use osiris_storage::StorageError;
use time::OffsetDateTime;

impl Core {
    pub async fn refresh(
        &self,
        id: &str,
        when: When,
        running: Option<RunningTask<'_>>,
    ) -> CoreResult<Refresh> {
        // Everything the refresh needs to read is read before the row is locked, so a refresh
        // holding a lock never waits for a second database connection.
        let unlocked = match self.store.connected_account(id).await {
            Ok(account) => account,
            Err(report)
                if running.is_some()
                    && matches!(report.current_context(), StorageError::NotFound(_)) =>
            {
                return Ok(Refresh::Stale);
            }
            Err(report) => return Err(report).switch(),
        };
        let config = self
            .store
            .auth_config(&unlocked.auth_config_id)
            .await
            .switch()?;
        let lock = self.store.lock_for_refresh(id).await.switch()?;
        let account = lock.account.clone();
        // A connection disabled because its last refresh could not schedule the next one: any
        // refresh task that arrives schedules it again.
        if running.is_some()
            && account.status == ConnectionStatus::Disabled
            && account.tokens.is_some()
            && account.refresh_task.is_none()
        {
            lock.release().await.switch()?;
            return Ok(match self.restore(&account).await? {
                Some(restored) => Refresh::Restored(restored),
                None => Refresh::Stale,
            });
        }
        if let Some(task) = running
            && (account.status != ConnectionStatus::Active
                || account.token_generation != task.generation)
        {
            lock.release().await.switch()?;
            return Ok(Refresh::Stale);
        }
        let due = account.tokens.as_ref().is_some_and(|tokens| {
            when == When::Now
                || tokens.needs_refresh(OffsetDateTime::now_utc(), self.settings.refresh.margin())
        });
        let refresh_token = account
            .tokens
            .as_ref()
            .and_then(|tokens| tokens.refresh_token.clone());
        let (true, ConnectionStatus::Active, Some(refresh_token)) =
            (due, account.status, refresh_token)
        else {
            lock.release().await.switch()?;
            return Ok(Refresh::Unchanged(account));
        };
        let provider = self
            .providers
            .get(&config.provider)
            .ok_or(DomainError::Internal)?;
        let redirect_uri = self.redirect_uri();
        let answer = oauth::refresh(
            provider.id(),
            &self.http,
            provider.oauth(),
            &oauth_client(&config, &redirect_uri),
            refresh_token.expose_secret(),
        )
        .await;
        let previous_task = account.refresh_task.clone();
        match answer {
            Ok(fresh) => self.save_refreshed(lock, new_tokens(fresh), running).await,
            Err(
                ProviderError::Refused { message, .. }
                | ProviderError::Unauthorized { message, .. },
            ) => {
                tracing::warn!(connected_account_id = id, %message, "the provider permanently refused the refresh; the user must reconnect");
                let event = lock
                    .expire("the provider refused the refresh; reconnect")
                    .await
                    .switch()?;
                self.publish(&event).await;
                self.cancel_previous(previous_task.as_deref(), running)
                    .await;
                Ok(Refresh::Ended(
                    self.store.connected_account(id).await.switch()?,
                ))
            }
            Err(error @ ProviderError::Unavailable { .. }) => {
                tracing::warn!(connected_account_id = id, %error, "the provider failed the refresh for now; it will be tried again");
                lock.failed("the provider could not be reached; retrying")
                    .await
                    .switch()?;
                Err(provider_error(error))
            }
        }
    }
}
