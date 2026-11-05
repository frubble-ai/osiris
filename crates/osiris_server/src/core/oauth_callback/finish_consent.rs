//! Finishes a consent. The `state` can be claimed only once. The code is exchanged for tokens, the
//! first refresh task is scheduled, and then the tokens and the task's name are saved together,
//! which makes the connection active. A provider that issues no refresh tokens needs no task. If
//! the provider should have sent a refresh token and didn't, or the first refresh task cannot be
//! scheduled, no tokens are kept and the connection is disabled, so the user must connect again.

use super::{Callback, Outcome, consent_error::consent_error_reason};
use crate::core::Core;
use osiris_domain::DomainError;
use osiris_tasks::names;

impl Core {
    pub async fn finish_consent(&self, callback: Callback) -> Outcome {
        let Some(state) = callback.state.filter(|state| !state.is_empty()) else {
            return Outcome::Unknown;
        };
        let claimed = match self.store.claim_consent(&state).await {
            Ok(Some(claimed)) => claimed,
            Ok(None) => return Outcome::Unknown,
            Err(error) => {
                tracing::error!(?error, "claiming a consent failed");
                return Outcome::Unknown;
            }
        };
        let account = claimed.account;
        // The consent is finishing, so its expiry task is no longer needed.
        self.cancel_quietly(&names::consent(&account.id)).await;
        let code = match (&callback.error, callback.code, claimed.expired) {
            (_, _, true) => {
                return self
                    .consent_failed(&account, "the consent took too long")
                    .await;
            }
            (Some(error), _, _) => {
                return self
                    .consent_failed(&account, &consent_error_reason(error))
                    .await;
            }
            (None, None, _) => {
                return self
                    .consent_failed(&account, "the provider sent no code")
                    .await;
            }
            (None, Some(code), false) => code,
        };
        let tokens = match self
            .exchange(&account, &code, claimed.pkce_verifier.as_deref())
            .await
        {
            Ok(tokens) => tokens,
            Err(report) => {
                tracing::warn!(error = ?report, "exchanging the consent's code failed");
                // A fixed reason: `last_error` reaches API clients, the report stays in the logs.
                let reason = match report.current_context() {
                    DomainError::ProviderRefused(_) => "the provider refused the consent's code",
                    DomainError::ProviderUnavailable(_) => "the provider could not be reached",
                    _ => "osiris could not finish the consent",
                };
                return self.consent_failed(&account, reason).await;
            }
        };
        self.activate_consent(&account, tokens).await
    }
}
