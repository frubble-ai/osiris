//! What a client is refused, and with which code: no key, a wrong key, the API key on an admin
//! route, an unknown version, an unknown provider, a missing field, and an id that isn't there.

use crate::testing::golden::{GoldenSession, Key};
use serde_json::json;

pub async fn run(session: &mut GoldenSession<'_>) {
    session
        .send("GET", "/v1/connected-accounts", Key::None, None, &[])
        .await;
    session
        .send("GET", "/v1/connected-accounts", Key::Wrong, None, &[])
        .await;
    session
        .send("GET", "/v1/auth-configs", Key::Api, None, &[])
        .await;
    session
        .send(
            "GET",
            "/v1/auth-configs",
            Key::Admin,
            None,
            &[("osiris-version", "2020-01-01.ra")],
        )
        .await;
    session
        .send(
            "POST",
            "/v1/auth-configs",
            Key::Admin,
            Some(json!({ "provider": "nowhere", "client_id": "c", "client_secret": "s" })),
            &[],
        )
        .await;
    session
        .send(
            "POST",
            "/v1/connected-accounts",
            Key::Api,
            Some(json!({ "user_id": "org_golden" })),
            &[],
        )
        .await;
    session
        .send(
            "GET",
            "/v1/connected-accounts/ca_000000000000000000000000",
            Key::Api,
            None,
            &[],
        )
        .await;
}
