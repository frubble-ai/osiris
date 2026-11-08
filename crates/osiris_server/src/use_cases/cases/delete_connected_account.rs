//! Disconnects: ends its triggers' subscriptions at the provider (marking them failed), ends the
//! grant at the provider whenever osiris holds tokens for it (active, expired or disabled),
//! deletes the connection, and cancels its pending refresh task by name. A
//! grant the provider no longer has counts as ended. If the provider cannot be reached, the
//! connection is kept so the call can be tried again. If cancelling the task fails, the task does
//! nothing when it runs, because the connection is gone.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::end_trigger_subscription::EndTriggerSubscriptionArgs,
    lib::{
        oauth::oauth_client::oauth_client,
        shared::{domain_errors::provider as provider_error, redirect_uri::redirect_uri},
        tasks::cancel_quietly::cancel_quietly,
        triggers::TriggerState,
    },
};
use futures::{StreamExt, TryStreamExt, stream};
use osiris_common::secret::ExposeSecret;
use osiris_domain::{DomainError, Trigger, TriggerStatus};
use osiris_interfaces::ProviderError;

/// What a trigger of a connection being deleted says, if the connection is kept after all.
const DISCONNECTING: &str =
    "its connection is being deleted; connect again and create the trigger again";

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
            let account = ctx.db.connected_account(id).await?;
            // Its triggers' subscriptions are ended first, while its token still reaches the
            // provider, and each trigger is marked failed: if the grant can't be ended below and
            // the connection is kept, its triggers say they no longer work. Deleting the
            // connection deletes them.
            let triggers = ctx.db.connection_triggers(id).await?;
            // Each trigger's subscription is its own request; a few are ended at once.
            stream::iter(triggers)
                .map(|trigger| async move { self.end_trigger(&trigger).await })
                .buffer_unordered(ctx.config.settings.providers.listing.concurrency.max(1))
                .try_collect::<Vec<()>>()
                .await?;
            // Whatever its status, a connection that holds tokens has a grant to end at the
            // provider.
            if let Some(tokens) = &account.tokens {
                let config = ctx.db.auth_config(&account.auth_config_id).await?;
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
            if let Some(task) = ctx.db.delete_connected_account(id).await? {
                cancel_quietly(ctx.scheduling(), &task).await;
            }
            Ok(())
        })
        .await
    }

    /// Ends `trigger`'s subscription at the provider and marks it failed.
    async fn end_trigger(&self, trigger: &Trigger) -> UseCaseResult<()> {
        self.end_trigger_subscription(EndTriggerSubscriptionArgs {
            trigger_id: &trigger.id,
            connected_account_id: &trigger.connected_account_id,
            provider: &trigger.provider,
            kind: &trigger.kind,
            config: &trigger.config,
            subscription_id: trigger.subscription_id.as_deref(),
            poll_task: trigger.poll_task.as_deref(),
        })
        .await?;
        let ended = TriggerState {
            status: TriggerStatus::Failed,
            checkpoint: trigger.checkpoint.clone(),
            last_error: Some(DISCONNECTING.into()),
            poll_task: None,
            poll_task_at: None,
        };
        self.context()
            .db
            .update_trigger_state(&trigger.id, trigger.poll_task.as_deref(), ended)
            .await?;
        Ok(())
    }
}
