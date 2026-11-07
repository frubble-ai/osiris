//! A request a route can't read is refused as `OS_02`, in osiris's own words for each reason: no
//! JSON content type, a body that isn't JSON, a body that doesn't match, a query that doesn't.

#![allow(
    clippy::unwrap_used,
    reason = "tests: a failure here is the test failing"
)]

use crate::testing::{ADMIN_KEY, API_KEY, setup_test_context};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::Value;
use sqlx::PgPool;
use tower::ServiceExt;

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_a_body_sent_without_a_json_content_type(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let request = Request::builder()
        .method("POST")
        .uri("/v1/auth-configs")
        .header("authorization", format!("Bearer {ADMIN_KEY}"))
        .body(Body::from(
            r#"{"provider":"github","client_id":"c","client_secret":"s"}"#,
        ))
        .unwrap();

    let answer = app.api.clone().oneshot(request).await.unwrap();

    assert_eq!(answer.status(), StatusCode::BAD_REQUEST);
    let bytes = answer.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["error"]["code"], "OS_02");
    assert_eq!(
        body["error"]["message"],
        "the body must be sent as application/json"
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn names_why_a_body_or_a_query_cannot_be_read(pool: PgPool) {
    let app = setup_test_context(pool).await;

    for (method, path, key, body, message) in [
        (
            "POST",
            "/v1/auth-configs",
            ADMIN_KEY,
            "not json",
            "the body is not valid JSON",
        ),
        (
            "POST",
            "/v1/auth-configs",
            ADMIN_KEY,
            r#"{"provider":"github"}"#,
            "the body does not match the request: a field is missing or has the wrong type",
        ),
        (
            "GET",
            "/v1/connected-accounts?limit=abc",
            API_KEY,
            "",
            "the query does not match the request: a parameter has the wrong type",
        ),
    ] {
        let refused = app
            .send_raw(method, path, Some(key), body.to_owned(), &[])
            .await;

        assert_eq!(refused.status, StatusCode::BAD_REQUEST, "{path}");
        assert_eq!(refused.body["error"]["code"], "OS_02", "{path}");
        assert_eq!(refused.body["error"]["message"], message, "{path}");
    }
}
