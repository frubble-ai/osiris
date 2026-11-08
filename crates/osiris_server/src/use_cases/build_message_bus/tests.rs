//! The bus writes events in the pinned release, an older one included, and refuses a pin osiris
//! doesn't serve.

use super::build_message_bus;
use crate::http::version::mocks::MOCK_RELEASES;
use osiris_events::{
    Message, adapters::memory::MemoryTransport,
    testing::generate_event_message::generate_event_message,
};
use serde_json::Value;
use std::sync::Arc;

#[tokio::test]
async fn writes_events_in_the_pinned_release_even_an_older_one() {
    let transport = Arc::new(MemoryTransport::default());
    let bus = build_message_bus(transport.clone(), MOCK_RELEASES, "2026-01-01.amun").unwrap();
    let event = generate_event_message();

    bus.send(Message::Event(event.clone())).await.unwrap();

    let sent: Value = serde_json::from_slice(&transport.sent().pop().unwrap().data).unwrap();
    assert_eq!(sent["osiris_version"], "2026-01-01.amun");
    // amun called the connection `connection_id`.
    assert_eq!(sent["connection_id"], event.connected_account_id);
}

#[test]
fn refuses_a_pin_osiris_does_not_serve() {
    let transport = Arc::new(MemoryTransport::default());
    let refused = build_message_bus(transport, MOCK_RELEASES, "1999-01-01.ra");
    let Err(error) = refused else {
        panic!("a pin osiris doesn't serve was accepted");
    };
    assert!(error.to_string().contains("1999-01-01.ra"));
}
