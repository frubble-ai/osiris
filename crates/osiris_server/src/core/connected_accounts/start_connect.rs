//! Opens a consent: creates a pending connection, schedules the task that fails the consent if
//! nobody finishes it in time, and returns the provider's consent URL for the browser. If that task
//! cannot be scheduled, the consent is marked as failed and an error is returned.

use crate::core::{Core, CoreResult, oauth_client::oauth_client};
use error_stack::ResultExt;
use osiris_common::{errors::ReportSwitchExt, ids};
use osiris_domain::{ConnectedAccount, DomainError};
use osiris_interfaces::oauth;
use osiris_storage::connected_accounts::NewPending;
use osiris_tasks::{ConsentTask, Task, TaskPayload, names};
use time::OffsetDateTime;
use url::Url;

pub struct StartConnect {
    pub auth_config_id: String,
    pub user_id: String,
    pub alias: Option<String>,
    pub callback_url: String,
}

impl Core {
    pub async fn start_connect(
        &self,
        start: StartConnect,
    ) -> CoreResult<(ConnectedAccount, String)> {
        let callback = Url::parse(&start.callback_url).change_context(DomainError::Invalid(
            "callback_url must be an absolute URL".into(),
        ))?;
        if !matches!(callback.scheme(), "http" | "https") {
            return Err(DomainError::Invalid("callback_url must be http or https".into()).into());
        }
        if start.user_id.trim().is_empty() {
            return Err(DomainError::Invalid("user_id is required".into()).into());
        }
        let config = self
            .store
            .auth_config(&start.auth_config_id)
            .await
            .switch()?;
        let provider = self
            .providers
            .get(&config.provider)
            .ok_or(DomainError::Internal)
            .attach_with(|| format!("no provider {:?}", config.provider))?;
        let state = ids::random(self.settings.consent.state_length);
        let expires_at = OffsetDateTime::now_utc() + self.settings.consent.ttl();
        let verifier = provider.oauth().pkce.then(oauth::pkce_verifier);
        let account = self
            .store
            .create_pending(NewPending {
                auth_config_id: config.id.clone(),
                user_id: start.user_id,
                alias: start.alias,
                callback_url: start.callback_url,
                oauth_state: state.clone(),
                oauth_state_expires_at: expires_at,
                pkce_verifier: verifier.clone(),
            })
            .await
            .switch()?;
        let expiry = Task {
            name: names::consent(&account.id),
            run_at: expires_at,
            payload: TaskPayload::ConsentExpire(ConsentTask {
                tenant_id: account.user_id.clone(),
                connected_account_id: account.id.clone(),
            }),
        };
        if let Err(report) = self.schedule(&expiry).await {
            if let Err(error) = self
                .store
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
        let redirect_uri = self.redirect_uri();
        let url = oauth::authorize_url(
            provider.oauth(),
            &oauth_client(&config, &redirect_uri),
            &state,
            verifier.as_deref(),
        );
        Ok((account, url.to_string()))
    }
}
