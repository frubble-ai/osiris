//! Disconnects: ends the grant at the provider whenever osiris holds tokens for it (active,
//! expired or disabled), deletes the connection, and cancels its pending refresh task by name. A
//! grant the provider no longer has counts as ended. If the provider cannot be reached, the
//! connection is kept so the call can be tried again. If cancelling the task fails, the task does
//! nothing when it runs, because the connection is gone.

use crate::use_cases::{
    UseCaseResult, UseCases,
    lib::{
        oauth::oauth_client::oauth_client,
        shared::{domain_errors::provider as provider_error, redirect_uri::redirect_uri},
        tasks::cancel_quietly::cancel_quietly,
    },
};
use osiris_common::{errors::ReportSwitchExt, secret::ExposeSecret};
use osiris_domain::DomainError;
use osiris_interfaces::ProviderError;

pub struct DeleteConnectedAccountArgs<'a> {
    pub id: &'a str,
}

impl UseCases {
    pub async fn delete_connected_account(
        &self,
        args: DeleteConnectedAccountArgs<'_>,
    ) -> UseCaseResult<()> {
        self.run("delete_connected_account", async {
            let ctx = self.context();
            let id = args.id;
            let account = ctx.store.connected_account(id).await.switch()?;
            // Whatever its status, a connection that holds tokens has a grant to end at the
            // provider.
            if let Some(tokens) = &account.tokens {
                let config = ctx.store.auth_config(&account.auth_config_id).await.switch()?;
                let provider = ctx
                    .providers
                    .get(&config.provider)
                    .ok_or(DomainError::Internal)?;
                let redirect_uri = redirect_uri(&ctx.config.public_url);
                match provider
                    .revoke(
                        &ctx.http,
                        &oauth_client(&config, &redirect_uri),
                        tokens.access_token.expose_secret(),
                    )
                    .await
                {
                    Ok(()) => {}
                    // The token is already dead there: nothing left to end.
                    Err(
                        error @ (ProviderError::Refused { .. }
                        | ProviderError::Unauthorized { .. }),
                    ) => {
                        tracing::info!(%error, connected_account_id = id, "revoke refused; the grant is gone");
                    }
                    Err(error) => return Err(provider_error(error)),
                }
            }
            if let Some(task) = ctx.store.delete_connected_account(id).await.switch()? {
                cancel_quietly(ctx.scheduling(), &task).await;
            }
            Ok(())
        })
        .await
    }
}
