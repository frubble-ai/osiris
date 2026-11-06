//! Sends a test event for a connection, its state left as it is: for a client to try its handling
//! (`osiris trigger`).

use crate::use_cases::{
    UseCaseResult, UseCases, cases::get_connected_account::GetConnectedAccountArgs,
    lib::shared::domain_errors::internal,
};
use osiris_domain::{Event, EventType};
use osiris_events::{Message, catalog::events::EventMessage};

pub struct TriggerEventArgs<'a> {
    pub event_type: EventType,
    pub connected_account_id: &'a str,
}

impl UseCases {
    pub async fn trigger_event(&self, args: TriggerEventArgs<'_>) -> UseCaseResult<Event> {
        self.run("trigger_event", async {
            // The connection must exist; its state is untouched.
            let account = self
                .get_connected_account(GetConnectedAccountArgs {
                    id: args.connected_account_id,
                })
                .await?;
            let event = Event::new(args.event_type, &account.id);
            self.context()
                .messages
                .send(Message::Event(EventMessage::from(&event)))
                .await
                .map_err(internal)?;
            Ok(event)
        })
        .await
    }
}
