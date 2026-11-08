//! Creates a trigger on a connection: the provider validates its config, the subscription is
//! created at the provider (a webhook, for example, delivering to osiris and signed with the
//! trigger's own secret), and, for a kind that checks on a schedule, its first check records the
//! state it starts from and its next check is scheduled.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::{
        get_connected_account::GetConnectedAccountArgs,
        run_connected_account_action::RunConnectedAccountActionArgs,
    },
    lib::{
        tasks::{next_trigger_check::next_trigger_check, schedule::schedule},
        triggers::NewTrigger,
    },
};
use error_stack::ResultExt;
use osiris_common::{
    ids,
    secret::{ExposeSecret, Secret, secret},
};
use osiris_domain::{DomainError, Trigger, TriggerStatus};
use osiris_interfaces::ActingAs;

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
            let (mut checkpoint, mut poll_task, mut poll_task_at) = (None, None, None);
            if implementation.trigger_checks_on_schedule(kind) {
                // The first check records where the trigger starts from; it fires nothing.
                let first = self
                    .run_connected_account_action(RunConnectedAccountActionArgs {
                        id: &account.id,
                        provider: &account.provider,
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
                checkpoint = Some(first.checkpoint);
                let task = next_trigger_check(
                    &id,
                    &account.id,
                    1,
                    ctx.config.settings.triggers.check_interval(),
                );
                schedule(ctx.scheduling(), &task).await?;
                poll_task_at = Some(task.run_at);
                poll_task = Some(task.name);
            }
            ctx.db
                .create_trigger(NewTrigger {
                    id,
                    connected_account_id: account.id,
                    kind: args.kind,
                    config: args.config,
                    secret: trigger_secret,
                    status: TriggerStatus::Active,
                    subscription_id,
                    checkpoint,
                    last_error: None,
                    poll_task,
                    poll_task_at,
                })
                .await
        })
        .await
    }
}
