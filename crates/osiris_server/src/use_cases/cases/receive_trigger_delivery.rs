//! Receives a delivery the provider sent a trigger: unless signed with the trigger's secret it is
//! refused as if the trigger didn't exist; an event the trigger fires on is published as a
//! `trigger.fired` event, its id the same for the same delivery; for a trigger that checks on a
//! schedule, a delivery about what it watches checks it now.

use crate::use_cases::{
    UseCaseResult, UseCases,
    cases::check_trigger::CheckTriggerArgs,
    lib::events::{publish::publish, trigger_event_id::trigger_event_id},
};
use axum::http::HeaderMap;
use osiris_common::{ids, secret::ExposeSecret};
use osiris_domain::{DomainError, Event, TriggerFired, TriggerStatus};
use osiris_interfaces::TriggerReceipt;

pub struct ReceiveTriggerDeliveryArgs<'a> {
    pub trigger_id: &'a str,
    pub headers: &'a HeaderMap,
    pub body: &'a [u8],
}

/// What became of a delivery.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delivery {
    /// Published as an event, or a check that ran.
    Taken,
    /// Signed, but nothing the trigger fires on.
    Ignored,
}

impl UseCases {
    pub async fn receive_trigger_delivery(
        &self,
        args: ReceiveTriggerDeliveryArgs<'_>,
    ) -> UseCaseResult<Delivery> {
        self.run("receive_trigger_delivery", async {
            let ctx = self.context();
            let trigger = ctx.db.trigger(args.trigger_id).await?;
            let implementation = ctx
                .providers
                .get(&trigger.provider)
                .ok_or(DomainError::Internal)?;
            let receipt = implementation.receive_trigger(
                &trigger.kind,
                &trigger.config,
                args.headers,
                args.body,
                trigger.secret.expose_secret(),
            );
            match receipt {
                // Said the same as a trigger that doesn't exist, so a guess learns nothing.
                TriggerReceipt::Unsigned => Err(DomainError::NotFound("trigger").into()),
                TriggerReceipt::Ignored(reason) => {
                    tracing::info!(trigger_id = %trigger.id, %reason, "delivery ignored");
                    Ok(Delivery::Ignored)
                }
                _ if trigger.status != TriggerStatus::Active => {
                    tracing::info!(trigger_id = %trigger.id, "delivery to a failed trigger ignored");
                    Ok(Delivery::Ignored)
                }
                TriggerReceipt::Fired { delivery_id, data } => {
                    let seen = delivery_id.unwrap_or_else(|| ids::random(24));
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
                    Ok(Delivery::Taken)
                }
                TriggerReceipt::Check => {
                    self.check_trigger(CheckTriggerArgs {
                        trigger_id: &trigger.id,
                        task: None,
                    })
                    .await?;
                    Ok(Delivery::Taken)
                }
            }
        })
        .await
    }
}
