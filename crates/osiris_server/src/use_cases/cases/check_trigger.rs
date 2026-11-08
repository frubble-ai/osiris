//! Checks a trigger that checks on a schedule (a watched item) at the provider: what changed since
//! its last check is published as a `trigger.fired` event, and the state it saw kept for the next.
//!
//! - On schedule, the next check is scheduled, and the state is written only while the trigger is
//!   still waiting for this check, so two checks at once never stop its schedule.
//! - A delivery that asks for a check writes only what it saw and leaves the schedule alone.
//! - A check the provider refuses for good (the item is gone) fails the trigger.
//! - A connection that can't be used now (disabled, expired, missing a scope, app credentials
//!   that can't mint its token) keeps the trigger active: it is checked again on schedule, and
//!   works again once the connection does.
//! - An outage is tried again.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::run_connected_account_action::RunConnectedAccountActionArgs,
    lib::{
        events::{publish::publish, trigger_event_id::trigger_event_id},
        installation_tokens::mint_refused::MintRefused,
        tasks::{next_trigger_check::next_trigger_check, schedule::schedule},
        triggers::TriggerState,
    },
};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_domain::{DomainError, Event, TaskOutcome, Trigger, TriggerFired, TriggerStatus};
use osiris_interfaces::ActingAs;

/// What a trigger whose check the provider refuses for good says, for the client.
const REFUSED: &str = "the provider refused the trigger's check; create it again";

/// What a check that waits for its connection says.
const WAITING: &str = "the connection can't be used now; the trigger is checked again on schedule";

pub struct CheckTriggerArgs<'a> {
    pub trigger_id: &'a str,
    /// The scheduled task this check runs as, and its number; none when a delivery asked for it.
    pub task: Option<(&'a str, i64)>,
}

/// How a check ended at the provider.
enum Checked {
    /// It answered: what it saw, and a change to publish, if any.
    Answered(osiris_interfaces::TriggerCheck),
    /// It refused for good: the trigger fails.
    Refused,
    /// The connection can't be used now: nothing changes but the schedule.
    Waiting,
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
                return Ok((
                    TaskOutcome::SkippedStale,
                    Some("the trigger is not waiting for this check"),
                ));
            }
            let checked = self.check_at_provider(&trigger).await?;
            let Some((name, check)) = args.task else {
                return self.after_delivery_check(&trigger, checked).await;
            };
            let (outcome, detail, state) = match checked {
                Checked::Refused => {
                    let state = TriggerState {
                        status: TriggerStatus::Failed,
                        checkpoint: trigger.checkpoint.clone(),
                        last_error: Some(REFUSED.into()),
                        poll_task: None,
                        poll_task_at: None,
                    };
                    (TaskOutcome::Failed, Some(REFUSED), state)
                }
                answered_or_waiting => {
                    let waiting = matches!(answered_or_waiting, Checked::Waiting);
                    let checkpoint = match answered_or_waiting {
                        Checked::Answered(answer) => {
                            self.publish_change(&trigger, answer.fired).await;
                            Some(answer.checkpoint)
                        }
                        Checked::Waiting | Checked::Refused => trigger.checkpoint.clone(),
                    };
                    let next = next_trigger_check(
                        &trigger.id,
                        &trigger.connected_account_id,
                        check.saturating_add(1),
                        ctx.config.settings.triggers.check_interval(),
                    );
                    schedule(ctx.scheduling(), &next).await?;
                    let state = TriggerState {
                        status: TriggerStatus::Active,
                        checkpoint,
                        last_error: waiting.then(|| WAITING.to_owned()),
                        poll_task: Some(next.name),
                        poll_task_at: Some(next.run_at),
                    };
                    (
                        TaskOutcome::TriggerChecked,
                        waiting.then_some(WAITING),
                        state,
                    )
                }
            };
            let written = ctx
                .db
                .update_trigger_state(&trigger.id, Some(name), state)
                .await?;
            if written.is_none() {
                // Another check moved the trigger on meanwhile; the next check it scheduled is
                // the one the trigger waits for, so the one scheduled here ends as stale.
                return Ok((
                    TaskOutcome::SkippedStale,
                    Some("another check ran meanwhile"),
                ));
            }
            Ok((outcome, detail))
        })
        .await
    }

    /// The check a delivery asked for: what it saw is written, and a refusal fails the trigger,
    /// unless a scheduled check moved it on meanwhile.
    async fn after_delivery_check(
        &self,
        trigger: &Trigger,
        checked: Checked,
    ) -> UseCaseResult<(TaskOutcome, Option<&'static str>)> {
        let ctx = self.context();
        match checked {
            Checked::Answered(answer) => {
                self.publish_change(trigger, answer.fired).await;
                ctx.db
                    .update_trigger_checkpoint(&trigger.id, &answer.checkpoint)
                    .await?;
                Ok((TaskOutcome::TriggerChecked, None))
            }
            Checked::Waiting => Ok((TaskOutcome::TriggerChecked, Some(WAITING))),
            Checked::Refused => {
                let state = TriggerState {
                    status: TriggerStatus::Failed,
                    checkpoint: trigger.checkpoint.clone(),
                    last_error: Some(REFUSED.into()),
                    poll_task: None,
                    poll_task_at: None,
                };
                ctx.db
                    .update_trigger_state(&trigger.id, trigger.poll_task.as_deref(), state)
                    .await?;
                Ok((TaskOutcome::Failed, Some(REFUSED)))
            }
        }
    }

    /// Asks the provider what the trigger watches looks like now.
    async fn check_at_provider(&self, trigger: &Trigger) -> UseCaseResult<Checked> {
        let ctx = self.context();
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
        match checked {
            Ok(answer) => Ok(Checked::Answered(answer)),
            // The provider answered for good: the item is gone or the config can't work.
            // Only the provider answering for good about what the trigger watches fails it; a
            // refused installation token is the app's credentials, which leave it waiting below.
            Err(report)
                if matches!(report.current_context(), DomainError::ProviderRefused(_))
                    && !report.contains::<MintRefused>() =>
            {
                tracing::warn!(error = ?report, trigger_id = %trigger.id, "the provider refuses the trigger's check; the trigger fails");
                Ok(Checked::Refused)
            }
            // A connection that can't be used now, or credentials that can't mint its token yet
            // (an auth config the operator can fix), leave the trigger waiting.
            Err(report)
                if report.contains::<MintRefused>()
                    || matches!(
                        report.current_context(),
                        DomainError::NotActive(_)
                            | DomainError::MissingScopes(_)
                            | DomainError::Invalid(_)
                    ) =>
            {
                tracing::info!(error = ?report, trigger_id = %trigger.id, "the trigger's connection can't be used now");
                Ok(Checked::Waiting)
            }
            Err(report) => Err(report),
        }
    }

    /// Publishes the change a check saw, if it saw one.
    async fn publish_change(&self, trigger: &Trigger, fired: Option<(String, serde_json::Value)>) {
        let Some((seen, data)) = fired else {
            return;
        };
        let ctx = self.context();
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
}
