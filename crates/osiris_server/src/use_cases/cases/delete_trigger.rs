//! Deletes a trigger: its subscription ended at the provider, its scheduled check cancelled, and
//! the trigger removed. A provider that can't be reached keeps the trigger, so the delete can be
//! tried again; a refusal, or a connection that no longer reaches the provider, is logged and the
//! trigger removed all the same.

use crate::use_cases::{
    UseCaseResult, UseCases, cases::end_trigger_subscription::EndTriggerSubscriptionArgs,
};

pub struct DeleteTriggerArgs<'a> {
    pub id: &'a str,
}

impl UseCases {
    pub async fn delete_trigger(&self, args: DeleteTriggerArgs<'_>) -> UseCaseResult<()> {
        self.run("delete_trigger", async {
            let ctx = self.context();
            let trigger = ctx.db.trigger(args.id).await?;
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
            ctx.db.delete_trigger(args.id).await
        })
        .await
    }
}
