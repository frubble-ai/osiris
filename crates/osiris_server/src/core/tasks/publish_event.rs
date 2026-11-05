//! Runs an `event.publish` task: publishes an event whose first publish failed. Clients ignore an
//! event they have already received, by its `event_id`.

use crate::core::{Core, CoreResult, errors::internal};
use osiris_domain::{Event, TaskOutcome};
use osiris_events::{Message, catalog::events::EventMessage};
use osiris_tasks::EventTask;

impl Core {
    pub(crate) async fn publish_event_task(
        &self,
        task: &EventTask,
    ) -> CoreResult<(TaskOutcome, Option<&'static str>)> {
        let event = Event::from(task);
        self.bus
            .send(Message::Event(EventMessage::from(&event)))
            .await
            .map_err(internal)?;
        Ok((TaskOutcome::Published, None))
    }
}
