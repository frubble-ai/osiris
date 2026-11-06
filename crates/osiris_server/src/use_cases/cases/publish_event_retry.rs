//! Runs an `event.publish` task: publishes an event whose first publish failed. Clients ignore an
//! event they have already received, by its `event_id`.

use crate::use_cases::{UseCaseResult, UseCases, lib::shared::domain_errors::internal};
use osiris_domain::{Event, TaskOutcome};
use osiris_events::{Message, catalog::events::EventMessage};
use osiris_tasks::EventTask;

pub struct PublishEventRetryArgs<'a> {
    pub task: &'a EventTask,
}

impl UseCases {
    pub async fn publish_event_retry(
        &self,
        args: PublishEventRetryArgs<'_>,
    ) -> UseCaseResult<(TaskOutcome, Option<&'static str>)> {
        self.run("publish_event_retry", async {
            let event = Event::from(args.task);
            self.context()
                .messages
                .send(Message::Event(EventMessage::from(&event)))
                .await
                .map_err(internal)?;
            Ok((TaskOutcome::Published, None))
        })
        .await
    }
}
