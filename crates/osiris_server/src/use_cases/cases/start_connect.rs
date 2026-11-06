//! Opens a consent: creates a pending connection, schedules the task that fails the consent if
//! nobody finishes it in time, and returns the provider's consent URL for the browser. If that task
//! cannot be scheduled, the consent is marked as failed and an error is returned.

use crate::use_cases::lib::connected_accounts::NewPending;
use crate::use_cases::{
    UseCaseResult, UseCases,
    lib::{
        oauth::oauth_client::oauth_client, shared::redirect_uri::redirect_uri,
        tasks::schedule::schedule,
    },
};
use error_stack::ResultExt;
use osiris_common::{
    ids,
    secret::{ExposeSecret, secret},
};
use osiris_domain::{ConnectedAccount, DomainError};
use osiris_interfaces::oauth;
use osiris_tasks::{ConsentTask, Task, TaskPayload, names};
use time::OffsetDateTime;
use url::Url;

pub struct StartConnectArgs {
    pub auth_config_id: String,
    pub user_id: String,
    pub alias: Option<String>,
    pub callback_url: String,
}

impl UseCases {
    /// The pending connection, and the provider's consent URL.
    pub async fn start_connect(
        &self,
        args: StartConnectArgs,
    ) -> UseCaseResult<(ConnectedAccount, String)> {
        self.run("start_connect", async {
            let ctx = self.context();
            let callback = Url::parse(&args.callback_url).change_context(DomainError::Invalid(
                "callback_url must be an absolute URL".into(),
            ))?;
            if !matches!(callback.scheme(), "http" | "https") {
                return Err(
                    DomainError::Invalid("callback_url must be http or https".into()).into(),
                );
            }
            if args.user_id.trim().is_empty() {
                return Err(DomainError::Invalid("user_id is required".into()).into());
            }
            let config = ctx.db.auth_config(&args.auth_config_id).await?;
            let provider = ctx
                .providers
                .get(&config.provider)
                .ok_or(DomainError::Internal)
                .attach_with(|| format!("no provider {:?}", config.provider))?;
            let consent = &ctx.config.settings.consent;
            let state = ids::random(consent.state_length);
            let expires_at = OffsetDateTime::now_utc() + consent.ttl();
            let verifier = provider
                .oauth()
                .pkce
                .then(|| secret(oauth::pkce_verifier()));
            let account = ctx
                .db
                .create_pending(NewPending {
                    auth_config_id: config.id.clone(),
                    user_id: args.user_id,
                    alias: args.alias,
                    callback_url: args.callback_url,
                    oauth_state: state.clone(),
                    oauth_state_expires_at: expires_at,
                    pkce_verifier: verifier.clone(),
                })
                .await?;
            let expiry = Task {
                name: names::consent(&account.id),
                run_at: expires_at,
                payload: TaskPayload::ConsentExpire(ConsentTask {
                    tenant_id: account.user_id.clone(),
                    connected_account_id: account.id.clone(),
                }),
            };
            if let Err(report) = schedule(ctx.scheduling(), &expiry).await {
                if let Err(error) = ctx
                    .db
                    .fail(
                        &account.id,
                        "osiris could not schedule the consent's expiry",
                    )
                    .await
                {
                    tracing::error!(?error, "marking the consent as failed did not work");
                }
                return Err(report);
            }
            let redirect_uri = redirect_uri(&ctx.config.public_url);
            let url = oauth::authorize_url(
                provider.oauth(),
                &oauth_client(&config, &redirect_uri),
                &state,
                verifier.as_ref().map(ExposeSecret::expose_secret),
            );
            Ok((account, url.to_string()))
        })
        .await
    }
}
