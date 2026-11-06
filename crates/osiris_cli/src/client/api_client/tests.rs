//! Tests of `api_client`.

#![allow(
    clippy::unwrap_used,
    reason = "tests: a failure here is the test failing"
)]

use super::*;
use crate::client::mocks::{
    generate_keys::{Keys, generate_keys},
    mock_osiris_api::MockOsirisApi,
};
use serde_json::json;
use wiremock::MockServer;

fn client_of(server: &MockServer, url_suffix: &str, keys: &Keys) -> Client {
    Client::new(Profile {
        url: format!("{}{url_suffix}", server.uri()),
        api_key: Some(keys.api.clone()),
        admin_key: Some(keys.admin.clone()),
        ..Profile::default()
    })
}

#[tokio::test]
async fn sends_the_api_key_and_the_version_and_reads_the_answer() {
    let server = MockServer::start().await;
    let keys = generate_keys();
    MockOsirisApi::on(&server)
        .answer(
            "GET",
            "/v1/connected-accounts",
            &keys.api,
            200,
            r#"{"data":[]}"#,
        )
        .await;

    let answer = client_of(&server, "", &keys)
        .call(Key::Api, Method::GET, "/v1/connected-accounts", None, &[])
        .await
        .unwrap();

    assert_eq!(answer, json!({ "data": [] }));
}

#[tokio::test]
async fn sends_the_admin_key_for_an_admin_call() {
    let server = MockServer::start().await;
    let keys = generate_keys();
    MockOsirisApi::on(&server)
        .answer("GET", "/v1/auth-configs", &keys.admin, 200, "{}")
        .await;

    let answer = client_of(&server, "", &keys)
        .call(Key::Admin, Method::GET, "/v1/auth-configs", None, &[])
        .await;

    assert!(answer.is_ok());
}

#[tokio::test]
async fn joins_the_path_to_a_url_with_a_trailing_slash() {
    let server = MockServer::start().await;
    let keys = generate_keys();
    MockOsirisApi::on(&server)
        .answer("GET", "/v1/auth-configs", &keys.admin, 200, "{}")
        .await;

    let answer = client_of(&server, "/", &keys)
        .call(Key::Admin, Method::GET, "/v1/auth-configs", None, &[])
        .await;

    assert!(answer.is_ok());
}

#[tokio::test]
async fn reads_an_empty_answer_as_null() {
    let server = MockServer::start().await;
    let keys = generate_keys();
    MockOsirisApi::on(&server)
        .answer("DELETE", "/v1/connected-accounts/ca_1", &keys.api, 204, "")
        .await;

    let answer = client_of(&server, "", &keys)
        .call(
            Key::Api,
            Method::DELETE,
            "/v1/connected-accounts/ca_1",
            None,
            &[],
        )
        .await
        .unwrap();

    assert_eq!(answer, Value::Null);
}

#[tokio::test]
async fn names_the_error_osiris_answered() {
    let server = MockServer::start().await;
    let keys = generate_keys();
    MockOsirisApi::on(&server)
        .answer(
            "GET",
            "/v1/connected-accounts/ca_1",
            &keys.api,
            404,
            r#"{"error":{"code":"OS_01","message":"connected account not found"}}"#,
        )
        .await;

    let error = client_of(&server, "", &keys)
        .call(
            Key::Api,
            Method::GET,
            "/v1/connected-accounts/ca_1",
            None,
            &[],
        )
        .await
        .unwrap_err()
        .to_string();

    assert!(error.contains("OS_01"), "{error}");
}

#[tokio::test]
async fn says_which_key_is_missing_without_calling() {
    let server = MockServer::start().await;
    let client = Client::new(Profile {
        url: server.uri(),
        ..Profile::default()
    });

    let api = client
        .call(Key::Api, Method::GET, "/v1/x", None, &[])
        .await
        .unwrap_err()
        .to_string();
    let admin = client
        .call(Key::Admin, Method::GET, "/v1/x", None, &[])
        .await
        .unwrap_err()
        .to_string();

    assert!(api.contains("API key"), "{api}");
    assert!(admin.contains("admin key"), "{admin}");
    assert!(server.received_requests().await.unwrap().is_empty());
}
