//! Send a test event for a connection, its state left as it is: for a client to try its handling
//! (`osiris trigger`).

use crate::core::{Core, CoreResult, errors::internal};
use osiris_domain::{Event, EventType};
use osiris_events::{Message, catalog::events::EventMessage};

impl Core {
    pub async fn trigger_event(
        &self,
        event_type: EventType,
        connected_account_id: &str,
    ) -> CoreResult<Event> {
        // The connection must exist; its state is untouched.
        let account = self.connected_account(connected_account_id).await?;
        let event = Event::new(event_type, &account.id);
        self.bus
            .send(Message::Event(EventMessage::from(&event)))
            .await
            .map_err(internal)?;
        Ok(event)
    }
}
