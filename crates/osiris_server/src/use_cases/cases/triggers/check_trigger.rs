//! Checks a trigger that checks on a schedule (a watched pull request) at the provider: what
//! changed since its last check is published as a `trigger.fired` event, and the state it saw
//! kept for the next. On schedule, the next check is scheduled; a delivery that asks for a check
//! leaves the schedule as it is. A check the provider refuses for good (the pull request is gone,
//! the connection no longer reaches it) fails the trigger; one that fails for now is tried again.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::{
        events::{publish::publish, trigger_event_id::trigger_event_id},
        tasks::{next_trigger_check::next_trigger_check, schedule::schedule},
        triggers::TriggerState,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_domain::{DomainError, Event, TaskOutcome, TriggerFired, TriggerStatus};
use osiris_interfaces::ActingAs;

/// What a trigger whose check the provider refuses for good says, for the client.
const REFUSED: &str = "the provider refused the trigger's check; create it again";

/// Whether a check's error ends the trigger: the provider refused, or the connection no longer
/// reaches it; an outage is tried again instead.
fn refused_for_good(error: &DomainError) -> bool {
    matches!(
        error,
        DomainError::ProviderRefused(_) | DomainError::NotActive(_) | DomainError::MissingScopes(_)
    )
}

pub struct CheckTriggerArgs<'a> {
    pub trigger_id: &'a str,
    /// The scheduled task this check runs as, and its number; none when a delivery asked for it.
    pub task: Option<(&'a str, i64)>,
}

impl UseCases {
    pub async fn check_trigger(
        &self,
        args: CheckTriggerArgs<'_>,
    ) -> UseCaseResult<(TaskOutcome, Option<&'static str>)> {
        self.run("check_trigger", async {
            let ctx = self.context();
            let trigger = match ctx.db.trigger(args.trigger_id).await {
                Ok(trigger) => trigger,
                Err(report) if matches!(report.current_context(), DomainError::NotFound(_)) => {
                    return Ok((TaskOutcome::SkippedStale, Some("the trigger was deleted")));
                }
                Err(report) => return Err(report),
            };
            let stale_task = args
                .task
                .is_some_and(|(name, _)| trigger.poll_task.as_deref() != Some(name));
            if trigger.status != TriggerStatus::Active || stale_task {
                return Ok((TaskOutcome::SkippedStale, Some("the trigger is not waiting for this check")));
            }
            let implementation = ctx
                .providers
                .get(&trigger.provider)
                .ok_or(DomainError::Internal)?;
            let (http, kind, config, checkpoint) = (
                &ctx.http,
                trigger.kind.as_str(),
                &trigger.config,
                trigger.checkpoint.as_ref(),
            );
            let checked = self
                .run_connected_account_action(RunConnectedAccountActionArgs {
                    id: &trigger.connected_account_id,
                    provider: &trigger.provider,
                    action: kind,
                    required_scopes: &[],
                    user_only: false,
                    call: |token: Secret, _acting: ActingAs| async move {
                        implementation
                            .check_trigger(http, token.expose_secret(), kind, config, checkpoint)
                            .await
                    },
                })
                .await;
            let check = match checked {
                Ok(check) => check,
                Err(report) if refused_for_good(report.current_context()) => {
                    tracing::warn!(error = ?report, trigger_id = %trigger.id, "the provider refuses the trigger's check; the trigger fails");
                    ctx.db
                        .update_trigger_state(
                            &trigger.id,
                            TriggerState {
                                status: TriggerStatus::Failed,
                                checkpoint: trigger.checkpoint.clone(),
                                last_error: Some(REFUSED.into()),
                                poll_task: None,
                                poll_task_at: None,
                            },
                        )
                        .await?;
                    return Ok((TaskOutcome::Failed, Some(REFUSED)));
                }
                Err(report) => return Err(report),
            };
            if let Some((seen, data)) = check.fired {
                let event = Event::trigger_fired(
                    trigger_event_id(&trigger.id, &seen),
                    &trigger.connected_account_id,
                    TriggerFired {
                        trigger_id: trigger.id.clone(),
                        trigger: trigger.kind.clone(),
                        data,
                    },
                );
                publish(&ctx.messages, ctx.scheduling(), &event).await;
            }
            let (mut poll_task, mut poll_task_at) = (trigger.poll_task, trigger.poll_task_at);
            if let Some((_, check)) = args.task {
                let task = next_trigger_check(
                    &trigger.id,
                    &trigger.connected_account_id,
                    check.saturating_add(1),
                    ctx.config.settings.triggers.check_interval(),
                );
                schedule(ctx.scheduling(), &task).await?;
                poll_task_at = Some(task.run_at);
                poll_task = Some(task.name);
            }
            ctx.db
                .update_trigger_state(
                    &trigger.id,
                    TriggerState {
                        status: TriggerStatus::Active,
                        checkpoint: Some(check.checkpoint),
                        last_error: None,
                        poll_task,
                        poll_task_at,
                    },
                )
                .await?;
            Ok((TaskOutcome::TriggerChecked, None))
        })
        .await
    }
}
