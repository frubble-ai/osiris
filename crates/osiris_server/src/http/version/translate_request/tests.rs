//! A request with no body or no query still meets the version changes, so one can add a default an
//! older release had; a request a change leaves alone keeps its empty body.

use super::translate_request;
use crate::testing::mock_releases::MOCK_RELEASES;
use axum::{body::Body, extract::Request};
use http_body_util::BodyExt;
use osiris_versioning::Route;
use serde_json::{Value, json};

async fn brought_up(method: &'static str, path: &'static str, uri: &str) -> (Request, Vec<u8>) {
    let route = Route { method, path };
    let request = Request::builder()
        .method(method)
        .uri(uri)
        .body(Body::empty())
        .expect("a request");
    let request = translate_request(MOCK_RELEASES, &MOCK_RELEASES[0], &route, 1024, request)
        .await
        .unwrap_or_else(|_| panic!("{method} {uri} is brought up"));
    let (parts, body) = request.into_parts();
    let bytes = body.collect().await.expect("a body").to_bytes().to_vec();
    (Request::from_parts(parts, Body::empty()), bytes)
}

#[tokio::test]
async fn gives_an_empty_body_the_default_an_older_release_meant() {
    let (request, body) = brought_up(
        "POST",
        "/v1/connected-accounts/{id}/refresh",
        "/v1/connected-accounts/ca_1/refresh",
    )
    .await;

    let body: Value = serde_json::from_slice(&body).expect("JSON");
    assert_eq!(body, json!({ "reason": "amun" }));
    assert_eq!(request.headers()["content-type"], "application/json");
}

#[tokio::test]
async fn gives_a_missing_query_the_default_an_older_release_meant() {
    let (request, _) = brought_up("GET", "/v1/auth-configs", "/v1/auth-configs").await;

    assert_eq!(request.uri().query(), Some("limit=1"));
}

#[tokio::test]
async fn leaves_a_request_no_change_touches_without_a_body_or_a_query() {
    let (request, body) = brought_up(
        "GET",
        "/v1/connected-accounts/{id}",
        "/v1/connected-accounts/ca_1",
    )
    .await;

    assert!(body.is_empty());
    assert_eq!(request.uri().query(), None);
    assert!(request.headers().get("content-type").is_none());
}
