//! A provider's app through its whole life: registered, read, listed, updated, deleted, and gone.

use crate::testing::golden::{GoldenSession, Key};
use serde_json::json;

pub async fn run(session: &mut GoldenSession<'_>) {
    let created = session
        .send(
            "POST",
            "/v1/auth-configs",
            Key::Admin,
            Some(json!({
                "provider": "github",
                "client_id": "golden-github-app",
                "client_secret": "golden-github-secret",
                "scopes": ["repo"],
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
            Some(json!({ "scopes": ["repo", "read:org"] })),
            &[],
        )
        .await;
    session.send("DELETE", &one, Key::Admin, None, &[]).await;
    session.send("GET", &one, Key::Admin, None, &[]).await;
}
