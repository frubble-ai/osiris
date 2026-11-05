//! Disconnects: ends the grant at the provider whenever osiris holds tokens for it (active,
//! expired or disabled), deletes the connection, and cancels its pending
//! refresh task by name. A grant the provider no longer has counts as ended. If the provider
//! cannot be reached, the connection is kept so the call can be tried again. If cancelling the
//! task fails, the task does nothing when it runs, because the connection is gone.

use crate::core::{
    Core, CoreResult, errors::provider as provider_error, oauth_client::oauth_client,
};
use osiris_common::{errors::ReportSwitchExt, secret::ExposeSecret};
use osiris_domain::DomainError;
use osiris_interfaces::ProviderError;

impl Core {
    pub async fn delete_connected_account(&self, id: &str) -> CoreResult<()> {
        let account = self.store.connected_account(id).await.switch()?;
        // Whatever its status, a connection that holds tokens has a grant to end at the provider.
        if let Some(tokens) = &account.tokens {
            let config = self
                .store
                .auth_config(&account.auth_config_id)
                .await
                .switch()?;
            let provider = self
                .providers
                .get(&config.provider)
                .ok_or(DomainError::Internal)?;
            let redirect_uri = self.redirect_uri();
            match provider
                .revoke(
                    &self.http,
                    &oauth_client(&config, &redirect_uri),
                    tokens.access_token.expose_secret(),
                )
                .await
            {
                Ok(()) => {}
                // The token is already dead there: nothing left to end.
                Err(
                    error @ (ProviderError::Refused { .. } | ProviderError::Unauthorized { .. }),
                ) => {
                    tracing::info!(%error, connected_account_id = id, "revoke refused; the grant is gone");
                }
                Err(error) => return Err(provider_error(error)),
            }
        }
        if let Some(task) = self.store.delete_connected_account(id).await.switch()? {
            self.cancel_quietly(&task).await;
        }
        Ok(())
    }
}
