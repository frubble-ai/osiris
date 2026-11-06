//! Tests of `start_consent`.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use super::*;
use crate::{
    client::mocks::{
        generate_keys::{Keys, generate_keys},
        mock_osiris_api::MockOsirisApi,
    },
    profile::Profile,
};
use serde_json::json;
use wiremock::MockServer;

fn client_of(server: &MockServer, keys: &Keys) -> Client {
    Client::new(Profile {
        url: server.uri(),
        api_key: Some(keys.api.clone()),
        admin_key: Some(keys.admin.clone()),
        ..Profile::default()
    })
}

async fn sent_bodies(server: &MockServer) -> Vec<Value> {
    server
        .received_requests()
        .await
        .unwrap()
        .iter()
        .filter(|request| request.method == Method::POST)
        .map(|request| serde_json::from_slice(&request.body).unwrap())
        .collect()
}

#[tokio::test]
async fn names_the_only_auth_config_of_the_provider_and_answers_the_consent_url() {
    let server = MockServer::start().await;
    let keys = generate_keys();
    let api = MockOsirisApi::on(&server);
    api.answer(
        "GET",
        "/v1/auth-configs",
        &keys.admin,
        200,
        r#"{"data":[{"id":"ac_slack","provider":"slack"},{"id":"ac_github","provider":"github"}]}"#,
    )
    .await;
    api.answer(
        "POST",
        "/v1/connected-accounts",
        &keys.api,
        201,
        r#"{"id":"ca_1","redirect_url":"https://github.com/login/oauth/authorize?state=s"}"#,
    )
    .await;

    let started = start_consent(
        &client_of(&server, &keys),
        "github",
        "org_1",
        None,
        "https://app.example/back",
    )
    .await
    .unwrap();

    assert_eq!(started.id, "ca_1");
    assert_eq!(
        started.consent_url,
        "https://github.com/login/oauth/authorize?state=s"
    );
    assert_eq!(
        sent_bodies(&server).await,
        [
            json!({ "auth_config_id": "ac_github", "user_id": "org_1", "callback_url": "https://app.example/back" })
        ]
    );
}

#[tokio::test]
async fn uses_the_named_auth_config_without_listing() {
    let server = MockServer::start().await;
    let keys = generate_keys();
    MockOsirisApi::on(&server)
        .answer(
            "POST",
            "/v1/connected-accounts",
            &keys.api,
            201,
            r#"{"id":"ca_2","redirect_url":"https://consent.example"}"#,
        )
        .await;

    start_consent(
        &client_of(&server, &keys),
        "github",
        "org_1",
        Some("ac_named".into()),
        "https://app.example/back",
    )
    .await
    .unwrap();

    assert_eq!(sent_bodies(&server).await[0]["auth_config_id"], "ac_named");
    assert_eq!(server.received_requests().await.unwrap().len(), 1);
}

#[tokio::test]
async fn fails_when_osiris_answers_no_consent_url() {
    let server = MockServer::start().await;
    let keys = generate_keys();
    MockOsirisApi::on(&server)
        .answer(
            "POST",
            "/v1/connected-accounts",
            &keys.api,
            201,
            r#"{"id":"ca_3"}"#,
        )
        .await;

    let error = start_consent(
        &client_of(&server, &keys),
        "github",
        "org_1",
        Some("ac_named".into()),
        "https://app.example/back",
    )
    .await
    .err()
    .unwrap()
    .to_string();

    assert!(error.contains("redirect_url"), "{error}");
}
