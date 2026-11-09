//! An event about a random connection.

use crate::catalog::events::EventMessage;
use fake::Fake;
use osiris_domain::EventType;

pub fn generate_event_message() -> EventMessage {
    let types = [
        EventType::ConnectionActive,
        EventType::ConnectionExpired,
        EventType::ConnectionRevoked,
    ];
    EventMessage {
        event_id: format!("ev_{}", 16.fake::<String>()),
        event_type: *types
            .get((0..types.len()).fake::<usize>())
            .unwrap_or(&EventType::ConnectionActive),
        connected_account_id: format!("ca_{}", 16.fake::<String>()),
        trigger_id: None,
        trigger: None,
        data: None,
    }
}
