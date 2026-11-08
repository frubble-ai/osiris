//! A provider's app through its whole life: registered, read, listed, updated, deleted, and gone.

use crate::testing::TestContext;
use crate::testing::golden::{GoldenSession, Key};
use osiris_providers::mocks::FAKE_ID;
use serde_json::json;

pub async fn run(_app: &TestContext, session: &mut GoldenSession<'_>) {
    let created = session
        .send(
            "POST",
            "/v1/auth-configs",
            Key::Admin,
            Some(json!({
                "provider": FAKE_ID,
                "client_id": "golden-app",
                "client_secret": "golden-secret",
                "scopes": ["read"],
            })),
            &[],
        )
        .await;
    let id = created.body["id"].as_str().expect("an id").to_owned();
    let one = format!("/v1/auth-configs/{id}");
    session.send("GET", &one, Key::Admin, None, &[]).await;
    session
        .send("GET", "/v1/auth-configs", Key::Admin, None, &[])
        .await;
    session
        .send(
            "PATCH",
            &one,
            Key::Admin,
            Some(json!({ "scopes": ["read", "write"] })),
            &[],
        )
        .await;
    session.send("DELETE", &one, Key::Admin, None, &[]).await;
    session.send("GET", &one, Key::Admin, None, &[]).await;
}

golden!("auth_configs", run);
