//! Finishes a consent. The `state` can be claimed only once. The code is exchanged for tokens, the
//! first refresh task is scheduled, and then the tokens and the task's name are saved together,
//! which makes the connection active. A provider that issues no refresh tokens needs no task. If
//! the provider should have sent a refresh token and didn't, or the first refresh task cannot be
//! scheduled, no tokens are kept and the connection is disabled, so the user must connect again.
//! It never fails: whatever happens, the browser has somewhere to go.

use crate::use_cases::{
    UseCaseResult, UseCases,
    lib::{
        consent::{
            Callback, Outcome,
            activate_consent::{Activation, activate_consent},
            consent_error_reason::consent_error_reason,
            consent_failed::consent_failed,
        },
        oauth::exchange_code::{Exchange, exchange_code},
        shared::redirect_uri::redirect_uri,
        tasks::cancel_quietly::cancel_quietly,
    },
};
use osiris_common::secret::ExposeSecret;
use osiris_domain::DomainError;
use osiris_tasks::names;

pub struct FinishConsentArgs {
    pub callback: Callback,
}

impl UseCases {
    pub async fn finish_consent(&self, args: FinishConsentArgs) -> UseCaseResult<Outcome> {
        self.run("finish_consent", async {
            let ctx = self.context();
            let callback = args.callback;
            let Some(state) = callback.state.filter(|state| !state.is_empty()) else {
                return Ok(Outcome::Unknown);
            };
            let claimed = match ctx.db.claim_consent(&state).await {
                Ok(Some(claimed)) => claimed,
                Ok(None) => return Ok(Outcome::Unknown),
                Err(error) => {
                    tracing::error!(?error, "claiming a consent failed");
                    return Ok(Outcome::Unknown);
                }
            };
            let account = claimed.account;
            // The consent is finishing, so its expiry task is no longer needed.
            cancel_quietly(ctx.scheduling(), &names::consent(&account.id)).await;
            let code = match (&callback.error, callback.code, claimed.expired) {
                (_, _, true) => {
                    return Ok(
                        consent_failed(&ctx.db, &account, "the consent took too long").await,
                    );
                }
                (Some(error), _, _) => {
                    return Ok(
                        consent_failed(&ctx.db, &account, &consent_error_reason(error)).await,
                    );
                }
                (None, None, _) => {
                    return Ok(
                        consent_failed(&ctx.db, &account, "the provider sent no code").await,
                    );
                }
                (None, Some(code), false) => code,
            };
            let exchanged = match ctx.db.auth_config(&account.auth_config_id).await {
                Ok(config) => {
                    let redirect_uri = redirect_uri(&ctx.config.public_url);
                    exchange_code(
                        &ctx.providers,
                        &ctx.http,
                        Exchange {
                            config: &config,
                            code: &code,
                            verifier: claimed
                                .pkce_verifier
                                .as_ref()
                                .map(ExposeSecret::expose_secret),
                            redirect_uri: &redirect_uri,
                        },
                    )
                    .await
                }
                Err(report) => Err(report),
            };
            let tokens = match exchanged {
                Ok(tokens) => tokens,
                Err(report) => {
                    tracing::warn!(error = ?report, "exchanging the consent's code failed");
                    // A fixed reason: `last_error` reaches API clients, the report stays in the logs.
                    let reason = match report.current_context() {
                        DomainError::ProviderRefused(_) => {
                            "the provider refused the consent's code"
                        }
                        DomainError::ProviderUnavailable(_) => "the provider could not be reached",
                        _ => "osiris could not finish the consent",
                    };
                    return Ok(consent_failed(&ctx.db, &account, reason).await);
                }
            };
            let issues_refresh_tokens = ctx
                .providers
                .get(&account.provider)
                .is_some_and(|provider| provider.oauth().issues_refresh_tokens);
            Ok(activate_consent(
                Activation {
                    db: &ctx.db,
                    messages: &ctx.messages,
                    scheduling: ctx.scheduling(),
                    refresh: &ctx.config.settings.refresh,
                    issues_refresh_tokens,
                },
                &account,
                tokens,
            )
            .await)
        })
        .await
    }
}
