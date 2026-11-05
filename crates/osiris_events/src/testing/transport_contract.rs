//! What every transport owes the use cases, checked the same way for each one: an event sent
//! through the bus reaches the transport whole, and a transport that can't publish says so, so the
//! use cases can schedule the event's retry.

use super::generate_event_message::generate_event_message;
use crate::{Message, MessageBus, Transport};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait TransportHarness: Send + Sync {
    fn transport(&self) -> Arc<dyn Transport>;

    /// Makes the transport unable to publish, when the harness can (`false` when it can't).
    async fn make_unavailable(&self) -> bool {
        false
    }
}

/// Runs every step of the contract against `harness`'s transport.
pub async fn transport_contract(harness: &dyn TransportHarness) {
    let bus = MessageBus::new(harness.transport());
    let event = generate_event_message();
    let sent = bus
        .send(Message::Event(event.clone()))
        .await
        .expect("an event is published");
    assert_eq!(
        sent.message,
        Message::Event(event),
        "the envelope carries the event whole"
    );
    if harness.make_unavailable().await {
        assert!(
            bus.send(Message::Event(generate_event_message()))
                .await
                .is_err(),
            "a transport that can't publish says so"
        );
    }
}
