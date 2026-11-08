//! An `Idempotency-Key`: a retry answered the first answer again, and the key refused for another
//! request.

use crate::testing::TestContext;
use crate::testing::golden::{GoldenSession, Key};
use osiris_providers::mocks::FAKE_ID;
use serde_json::json;

pub async fn run(_app: &TestContext, session: &mut GoldenSession<'_>) {
    let body = json!({
        "provider": FAKE_ID,
        "client_id": "golden-app",
        "client_secret": "golden-secret",
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
                "provider": FAKE_ID,
                "client_id": "golden-other-app",
                "client_secret": "golden-other-secret",
            })),
            &keyed,
        )
        .await;
    session
        .send("GET", "/v1/auth-configs", Key::Admin, None, &[])
        .await;
}

golden!("idempotency", run);
