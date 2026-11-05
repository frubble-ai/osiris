//! The code was exchanged for tokens: schedules the first refresh when the provider issues refresh
//! tokens, then saves the tokens and the task's name together, which makes the connection active.
//! A provider that should have sent a refresh token and didn't, or a first refresh that cannot be
//! scheduled, disables the connection without keeping the tokens.

use super::{
    Outcome,
    back_to::back_to,
    first_refresh::{FirstRefresh, first_refresh},
};
use crate::core::{Core, refresh::scheduled::scheduled};
use osiris_domain::{ConnectedAccount, ConnectionStatus, Event, EventType};
use osiris_storage::connected_accounts::NewTokens;

impl Core {
    pub(crate) async fn activate_consent(
        &self,
        account: &ConnectedAccount,
        tokens: NewTokens,
    ) -> Outcome {
        let issues_refresh_tokens = self
            .providers
            .get(&account.provider)
            .is_some_and(|provider| provider.oauth().issues_refresh_tokens);
        let refresh = match first_refresh(issues_refresh_tokens, tokens.refresh_token.is_some()) {
            FirstRefresh::Schedule => Some(self.next_refresh(
                account,
                account.token_generation + 1,
                tokens.access_token_expires_at,
            )),
            FirstRefresh::NotNeeded => None,
            FirstRefresh::MissingRefreshToken => {
                return self
                    .disable_consent(
                        account,
                        "the provider sent no refresh token; check that the app issues them (expiring tokens or token rotation), then connect again",
                    )
                    .await;
            }
        };
        if let Some(task) = &refresh
            && let Err(report) = self.schedule(task).await
        {
            tracing::warn!(error = ?report, connected_account_id = %account.id, "the first refresh could not be scheduled; the connection is disabled");
            return self
                .disable_consent(
                    account,
                    "osiris could not schedule the token refresh; connect again",
                )
                .await;
        }
        let saved = self
            .store
            .activate(account, &tokens, refresh.as_ref().map(scheduled).as_ref())
            .await;
        match saved {
            Ok(active) => {
                self.publish(&Event::new(EventType::ConnectionActive, &active.id))
                    .await;
                tracing::info!(connected_account_id = %active.id, "connection active");
                Outcome::Redirect(back_to(
                    &active.callback_url,
                    &active.id,
                    ConnectionStatus::Active,
                ))
            }
            Err(error) => {
                tracing::error!(?error, "saving the tokens failed");
                if let Some(task) = &refresh {
                    self.cancel_quietly(&task.name).await;
                }
                self.consent_failed(account, "osiris could not save the tokens")
                    .await
            }
        }
    }
}
