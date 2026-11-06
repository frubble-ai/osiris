//! Tests of `api_client`.

#![allow(
    clippy::unwrap_used,
    reason = "tests: a failure here is the test failing"
)]

use super::*;
use crate::client::mocks::mock_osiris_api::MockOsirisApi;
use serde_json::json;
use wiremock::MockServer;

const API_KEY: &str = "the-api-key";
const ADMIN_KEY: &str = "the-admin-key";

fn client_of(server: &MockServer, url_suffix: &str) -> Client {
    Client::new(Profile {
        url: format!("{}{url_suffix}", server.uri()),
        api_key: Some(API_KEY.into()),
        admin_key: Some(ADMIN_KEY.into()),
        ..Profile::default()
    })
}

#[tokio::test]
async fn sends_the_api_key_and_the_version_and_reads_the_answer() {
    let server = MockServer::start().await;
    MockOsirisApi::on(&server)
        .answer(
            "GET",
            "/v1/connected-accounts",
            API_KEY,
            200,
            r#"{"data":[]}"#,
        )
        .await;

    let answer = client_of(&server, "")
        .call(Key::Api, Method::GET, "/v1/connected-accounts", None, &[])
        .await
        .unwrap();

    assert_eq!(answer, json!({ "data": [] }));
}

#[tokio::test]
async fn sends_the_admin_key_for_an_admin_call() {
    let server = MockServer::start().await;
    MockOsirisApi::on(&server)
        .answer("GET", "/v1/auth-configs", ADMIN_KEY, 200, "{}")
        .await;

    let answer = client_of(&server, "")
        .call(Key::Admin, Method::GET, "/v1/auth-configs", None, &[])
        .await;

    assert!(answer.is_ok());
}

#[tokio::test]
async fn joins_the_path_to_a_url_with_a_trailing_slash() {
    let server = MockServer::start().await;
    MockOsirisApi::on(&server)
        .answer("GET", "/v1/auth-configs", ADMIN_KEY, 200, "{}")
        .await;

    let answer = client_of(&server, "/")
        .call(Key::Admin, Method::GET, "/v1/auth-configs", None, &[])
        .await;

    assert!(answer.is_ok());
}

#[tokio::test]
async fn reads_an_empty_answer_as_null() {
    let server = MockServer::start().await;
    MockOsirisApi::on(&server)
        .answer("DELETE", "/v1/connected-accounts/ca_1", API_KEY, 204, "")
        .await;

    let answer = client_of(&server, "")
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
    MockOsirisApi::on(&server)
        .answer(
            "GET",
            "/v1/connected-accounts/ca_1",
            API_KEY,
            404,
            r#"{"error":{"code":"OS_01","message":"connected account not found"}}"#,
        )
        .await;

    let error = client_of(&server, "")
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
