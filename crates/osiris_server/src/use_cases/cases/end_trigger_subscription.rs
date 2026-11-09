//! Ends what a trigger subscribed to at its provider (a webhook, for example) through its
//! connection, and cancels its scheduled check. A provider that can't be reached fails the call,
//! so the caller tries again with the trigger still there. A refusal, or a connection that can't
//! be used, is logged and nothing more: no retry would end the subscription, which then can only
//! be ended at the provider by hand.

use crate::use_cases::{
    UseCaseResult, UseCases, cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::tasks::cancel_quietly::cancel_quietly,
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_domain::DomainError;
use osiris_interfaces::ActingAs;

pub struct EndTriggerSubscriptionArgs<'a> {
    pub trigger_id: &'a str,
    pub connected_account_id: &'a str,
    pub provider: &'a str,
    pub kind: &'a str,
    pub config: &'a serde_json::Value,
    /// The provider's id of what the trigger subscribed to; none when it subscribed to nothing.
    pub subscription_id: Option<&'a str>,
    pub poll_task: Option<&'a str>,
}

impl UseCases {
    pub async fn end_trigger_subscription(
        &self,
        args: EndTriggerSubscriptionArgs<'_>,
    ) -> UseCaseResult<()> {
        self.run("end_trigger_subscription", async {
            let ctx = self.context();
            if let (Some(subscription_id), Some(implementation)) =
                (args.subscription_id, ctx.providers.get(args.provider))
            {
                let (http, kind, config) = (&ctx.http, args.kind, args.config);
                let ended = self
                    .run_connected_account_action(RunConnectedAccountActionArgs {
                        id: args.connected_account_id,
                        provider: args.provider,
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
                match ended {
                    Ok(()) => {}
                    // The provider can't be reached now: the trigger stays as it is, so the
                    // caller can try again and its subscription isn't left behind.
                    Err(report)
                        if matches!(
                            report.current_context(),
                            DomainError::ProviderUnavailable(_)
                        ) =>
                    {
                        return Err(report);
                    }
                    // Refused for good, or a connection that can't be used: nothing osiris can
                    // retry ends it, so it is logged and the trigger removed all the same.
                    Err(report) => {
                        tracing::warn!(error = ?report, trigger_id = args.trigger_id, "the trigger's subscription could not be ended at the provider");
                    }
                }
            }
            if let Some(task) = args.poll_task {
                cancel_quietly(ctx.scheduling(), task).await;
            }
            Ok(())
        })
        .await
    }
}
