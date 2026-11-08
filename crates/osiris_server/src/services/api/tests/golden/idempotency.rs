//! An `Idempotency-Key`: a retry answered the first answer again, and the key refused for another
//! request.

use crate::testing::golden::{GoldenSession, Key};
use serde_json::json;

pub async fn run(session: &mut GoldenSession<'_>) {
    let body = json!({
        "provider": "linear",
        "client_id": "golden-linear-app",
        "client_secret": "golden-linear-secret",
    });
    let keyed = [("idempotency-key", "golden-key")];
    session
        .send(
            "POST",
            "/v1/auth-configs",
            Key::Admin,
            Some(body.clone()),
            &keyed,
        )
        .await;
    session
        .send("POST", "/v1/auth-configs", Key::Admin, Some(body), &keyed)
        .await;
    session
        .send(
            "POST",
            "/v1/auth-configs",
            Key::Admin,
            Some(json!({
                "provider": "slack",
                "client_id": "golden-slack-app",
                "client_secret": "golden-slack-secret",
            })),
            &keyed,
        )
        .await;
    session
        .send("GET", "/v1/auth-configs", Key::Admin, None, &[])
        .await;
}
