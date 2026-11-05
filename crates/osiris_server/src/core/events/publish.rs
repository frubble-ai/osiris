//! Publishes one event. If the bus fails, schedules a task, named after the event, that publishes
//! it again. The event is lost only if that task cannot be scheduled either, and this is logged.

use crate::core::Core;
use osiris_domain::Event;
use osiris_events::{Message, catalog::events::EventMessage};
use osiris_tasks::{EventTask, Task, TaskPayload, names};
use time::OffsetDateTime;

impl Core {
    pub async fn publish(&self, event: &Event) {
        let Err(error) = self
            .bus
            .send(Message::Event(EventMessage::from(event)))
            .await
        else {
            return;
        };
        tracing::warn!(%error, event_id = %event.id, "publishing the event failed; scheduling a task to publish it again");
        let task = Task {
            name: names::event(&event.id),
            run_at: OffsetDateTime::now_utc(),
            payload: TaskPayload::EventPublish(EventTask::from(event)),
        };
        if let Err(report) = self.schedule(&task).await {
            tracing::error!(error = ?report, event_id = %event.id, "the retry task could not be scheduled; the event is lost");
        }
    }
}
