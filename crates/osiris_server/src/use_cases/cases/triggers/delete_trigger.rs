//! Deletes a trigger: its subscription ended at the provider when its connection still reaches it
//! (a failure there is logged, and the trigger deleted all the same), its scheduled check
//! cancelled, and the trigger removed.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        get_connected_account::GetConnectedAccountArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
    lib::tasks::cancel_quietly::cancel_quietly,
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_domain::DomainError;
use osiris_interfaces::ActingAs;

pub struct DeleteTriggerArgs<'a> {
    pub id: &'a str,
}

impl UseCases {
    pub async fn delete_trigger(&self, args: DeleteTriggerArgs<'_>) -> UseCaseResult<()> {
        self.run("delete_trigger", async {
            let ctx = self.context();
            let trigger = ctx.db.trigger(args.id).await?;
            let account = self
                .get_connected_account(GetConnectedAccountArgs {
                    id: &trigger.connected_account_id,
                })
                .await?;
            let implementation = ctx
                .providers
                .get(&account.provider)
                .ok_or(DomainError::Internal)?;
            if let Some(subscription_id) = trigger.subscription_id.as_deref() {
                let (http, kind, config) = (&ctx.http, trigger.kind.as_str(), &trigger.config);
                let ended = self
                    .run_connected_account_action(RunConnectedAccountActionArgs {
                        id: &account.id,
                        provider: &account.provider,
                        action: kind,
                        required_scopes: &[],
                        user_only: false,
                        call: |token: Secret, _acting: ActingAs| async move {
                            implementation
                                .unsubscribe_trigger(
                                    http,
                                    token.expose_secret(),
                                    kind,
                                    config,
                                    subscription_id,
                                )
                                .await
                        },
                    })
                    .await;
                if let Err(report) = ended {
                    tracing::warn!(error = ?report, trigger_id = args.id, "the trigger's subscription could not be ended at the provider; deleting the trigger all the same");
                }
            }
            if let Some(task) = trigger.poll_task.as_deref() {
                cancel_quietly(ctx.scheduling(), task).await;
            }
            ctx.db.delete_trigger(args.id).await
        })
        .await
    }
}
