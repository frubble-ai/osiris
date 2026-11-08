//! Creates a trigger on a connection: the provider validates its config, the subscription is
//! created at the provider (a webhook, for example, delivering to osiris and signed with the
//! trigger's own secret), and, for a kind that checks on a schedule, its first check records the
//! state it starts from and its next check is scheduled. If anything fails once the subscription
//! exists, the subscription is ended again, so none is left at the provider without a trigger.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        end_trigger_subscription::EndTriggerSubscriptionArgs,
        get_connected_account::GetConnectedAccountArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
    lib::{
        tasks::{
            cancel_quietly::cancel_quietly, next_trigger_check::next_trigger_check,
            schedule::schedule,
        },
        triggers::NewTrigger,
    },
};
use error_stack::ResultExt;
use osiris_common::{
    ids,
    secret::{ExposeSecret, Secret, secret},
};
use osiris_domain::{DomainError, Trigger, TriggerStatus};
use osiris_interfaces::{ActingAs, Provider};

/// How long a trigger's secret is, in characters: as long as a consent's state at its longest.
const SECRET_LENGTH: usize = 48;

pub struct CreateTriggerArgs {
    pub connected_account_id: String,
    /// Its kind, `<provider>.<kind>`.
    pub kind: String,
    pub config: serde_json::Value,
}

impl UseCases {
    pub async fn create_trigger(&self, args: CreateTriggerArgs) -> UseCaseResult<Trigger> {
        self.run("create_trigger", async {
            let ctx = self.context();
            let account = self
                .get_connected_account(GetConnectedAccountArgs {
                    id: &args.connected_account_id,
                })
                .await?;
            let implementation = ctx
                .providers
                .get(&account.provider)
                .ok_or(DomainError::Internal)
                .attach_with(|| format!("no provider {:?}", account.provider))?;
            implementation
                .validate_trigger(&args.kind, &args.config)
                .map_err(DomainError::Invalid)?;
            let id = ids::id("tr");
            let trigger_secret = secret(ids::random(SECRET_LENGTH));
            let callback_url = format!("{}/webhooks/triggers/{id}", ctx.config.public_url);
            let (http, kind, config) = (&ctx.http, args.kind.as_str(), &args.config);
            let subscription_id = self
                .run_connected_account_action(RunConnectedAccountActionArgs {
                    id: &account.id,
                    provider: &account.provider,
                    action: kind,
                    required_scopes: &[],
                    user_only: false,
                    call: |token: Secret, _acting: ActingAs| {
                        let (callback_url, trigger_secret) = (&callback_url, &trigger_secret);
                        async move {
                            implementation
                                .subscribe_trigger(
                                    http,
                                    token.expose_secret(),
                                    kind,
                                    config,
                                    callback_url,
                                    trigger_secret.expose_secret(),
                                )
                                .await
                        }
                    },
                })
                .await?;
            let new_trigger = NewTrigger {
                id,
                connected_account_id: account.id,
                kind: args.kind,
                config: args.config,
                secret: trigger_secret,
                status: TriggerStatus::Active,
                subscription_id,
                checkpoint: None,
                last_error: None,
                poll_task: None,
                poll_task_at: None,
            };
            let (trigger_id, connected_account_id, provider) = (
                new_trigger.id.clone(),
                new_trigger.connected_account_id.clone(),
                account.provider.clone(),
            );
            let (kind, config, subscription_id) = (
                new_trigger.kind.clone(),
                new_trigger.config.clone(),
                new_trigger.subscription_id.clone(),
            );
            match self
                .start_trigger(implementation, &provider, new_trigger)
                .await
            {
                Ok(trigger) => Ok(trigger),
                Err(report) => {
                    // Nothing will know of the subscription: it is ended now, not left behind.
                    let ended = self
                        .end_trigger_subscription(EndTriggerSubscriptionArgs {
                            trigger_id: &trigger_id,
                            connected_account_id: &connected_account_id,
                            provider: &provider,
                            kind: &kind,
                            config: &config,
                            subscription_id: subscription_id.as_deref(),
                            poll_task: None,
                        })
                        .await;
                    if let Err(cleanup) = ended {
                        tracing::warn!(
                            error = ?cleanup,
                            trigger_id = %trigger_id,
                            "the subscription of a trigger whose creation failed could not be ended"
                        );
                    }
                    Err(report)
                }
            }
        })
        .await
    }

    /// After the subscription: for a kind that checks on a schedule, its first check (which
    /// records where it starts from and fires nothing) and its next check scheduled; then the
    /// trigger saved. A scheduled check is cancelled if the trigger can't be saved.
    async fn start_trigger(
        &self,
        implementation: &dyn Provider,
        provider: &str,
        mut trigger: NewTrigger,
    ) -> UseCaseResult<Trigger> {
        let ctx = self.context();
        let (http, kind, config) = (&ctx.http, trigger.kind.as_str(), &trigger.config);
        if implementation.trigger_checks_on_schedule(kind) {
            let first = self
                .run_connected_account_action(RunConnectedAccountActionArgs {
                    id: &trigger.connected_account_id,
                    provider,
                    action: kind,
                    required_scopes: &[],
                    user_only: false,
                    call: |token: Secret, _acting: ActingAs| async move {
                        implementation
                            .check_trigger(http, token.expose_secret(), kind, config, None)
                            .await
                    },
                })
                .await?;
            let task = next_trigger_check(
                &trigger.id,
                &trigger.connected_account_id,
                1,
                ctx.config.settings.triggers.check_interval(),
            );
            schedule(ctx.scheduling(), &task).await?;
            trigger.checkpoint = Some(first.checkpoint);
            trigger.poll_task_at = Some(task.run_at);
            trigger.poll_task = Some(task.name);
        }
        let poll_task = trigger.poll_task.clone();
        let created = ctx.db.create_trigger(trigger).await;
        if created.is_err()
            && let Some(task) = poll_task.as_deref()
        {
            cancel_quietly(ctx.scheduling(), task).await;
        }
        created
    }
}
