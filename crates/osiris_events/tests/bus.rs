//! The bus through the memory transport.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "test fixtures: a failure here is the test failing"
)]

use osiris_domain::EventType;
use osiris_events::{
    Channel, Message, MessageBus, adapters::memory::MemoryTransport, catalog::events::EventMessage,
};
use std::sync::Arc;

#[tokio::test]
async fn sends_each_message_wrapped_on_its_channel() {
    let memory = Arc::new(MemoryTransport::default());
    let bus = MessageBus::new(memory.clone());
    bus.send(Message::Event(EventMessage {
        event_id: "ev_1".into(),
        event_type: EventType::ConnectionExpired,
        connected_account_id: "ca_1".into(),
    }))
    .await
    .unwrap();

    let published = memory.published();
    assert_eq!(published.len(), 1);
    assert_eq!(published[0].0, Channel::Events);
    assert_eq!(published[0].1.version, 1);
    assert!(published[0].1.id.starts_with("msg_"));
}

#[tokio::test]
async fn fails_while_the_transport_does() {
    let memory = Arc::new(MemoryTransport::default());
    let bus = MessageBus::new(memory.clone());
    memory.fail(true);
    let event = || {
        Message::Event(EventMessage {
            event_id: "ev_1".into(),
            event_type: EventType::ConnectionActive,
            connected_account_id: "ca_1".into(),
        })
    };
    assert!(bus.send(event()).await.is_err());
    memory.fail(false);
    bus.send(event()).await.unwrap();
    assert_eq!(memory.published().len(), 1);
}
