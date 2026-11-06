//! Tests of `trigger`.

use super::*;
use serde_json::json;

#[test]
fn sends_the_event_type_and_the_connection() {
    let trigger = Trigger {
        event: "connection.expired".into(),
        connection: "ca_1".into(),
    };

    assert_eq!(
        trigger.body(),
        json!({ "type": "connection.expired", "connected_account_id": "ca_1" })
    );
}
