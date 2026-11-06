//! Publishes one event. If the bus fails, schedules a task, named after the event, that publishes
//! it again. The event is lost only if that task cannot be scheduled either, and this is logged.

use crate::use_cases::lib::tasks::{Scheduling, schedule::schedule};
use osiris_domain::Event;
use osiris_events::{Message, MessageBus, catalog::events::EventMessage};
use osiris_tasks::{EventTask, Task, TaskPayload, names};
use time::OffsetDateTime;

pub async fn publish(messages: &MessageBus, scheduling: Scheduling<'_>, event: &Event) {
    let Err(error) = messages
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
    if let Err(report) = schedule(scheduling, &task).await {
        tracing::error!(error = ?report, event_id = %event.id, "the retry task could not be scheduled; the event is lost");
    }
}
