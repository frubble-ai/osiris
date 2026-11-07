//! Test events of every type, as a client triggers them to try its handling, and one for a
//! connection that isn't there.

use crate::testing::TestContext;
use crate::testing::golden::{GoldenSession, Key};
use serde_json::json;

pub async fn run(app: &TestContext, session: &mut GoldenSession<'_>) {
    let id = app.connected(28800).await;
    // The connection's own `connection.active` is arranged, not part of the transcript.
    session.skip_events();
    for event_type in [
        "connection.active",
        "connection.expired",
        "connection.revoked",
    ] {
        session
            .send(
                "POST",
                "/v1/events/trigger",
                Key::Admin,
                Some(json!({ "type": event_type, "connected_account_id": id })),
                &[],
            )
            .await;
    }
    session
        .send(
            "POST",
            "/v1/events/trigger",
            Key::Admin,
            Some(json!({
                "type": "connection.active",
                "connected_account_id": "ca_000000000000000000000000",
            })),
            &[],
        )
        .await;
}
