//! Test events through the router.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use crate::testing::{ADMIN_KEY, API_KEY, setup_test_context};
use axum::http::StatusCode;
use osiris_domain::EventType;
use serde_json::json;
use sqlx::PgPool;

async fn triggers_and_leaves_the_connection_as_it_is(
    pool: PgPool,
    event_type: &str,
    expected: EventType,
) {
    let app = setup_test_context(pool).await;
    let id = app.connected(28800).await;
    let before = app.events().len();

    let answer = app
        .send(
            "POST",
            "/v1/events/trigger",
            Some(ADMIN_KEY),
            Some(json!({ "type": event_type, "connected_account_id": id })),
            &[],
        )
        .await;

    assert_eq!(answer.status, StatusCode::CREATED, "{}", answer.body);
    assert_eq!(app.events().last().unwrap().event_type, expected);
    assert_eq!(app.events().len(), before + 1);
    assert_eq!(app.account(&id).await["status"], "active");
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn triggers_connection_active(pool: PgPool) {
    triggers_and_leaves_the_connection_as_it_is(
        pool,
        "connection.active",
        EventType::ConnectionActive,
    )
    .await;
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn triggers_connection_expired(pool: PgPool) {
    triggers_and_leaves_the_connection_as_it_is(
        pool,
        "connection.expired",
        EventType::ConnectionExpired,
    )
    .await;
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn triggers_connection_revoked(pool: PgPool) {
    triggers_and_leaves_the_connection_as_it_is(
        pool,
        "connection.revoked",
        EventType::ConnectionRevoked,
    )
    .await;
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_an_unknown_event_type_or_connection(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected(28800).await;
    let unknown_type = app
        .send(
            "POST",
            "/v1/events/trigger",
            Some(ADMIN_KEY),
            Some(json!({ "type": "connection.nope", "connected_account_id": id })),
            &[],
        )
        .await;
    assert_eq!(unknown_type.status, StatusCode::BAD_REQUEST);
    let unknown_connection = app
        .send(
            "POST",
            "/v1/events/trigger",
            Some(ADMIN_KEY),
            Some(json!({ "type": "connection.active", "connected_account_id": "ca_missing" })),
            &[],
        )
        .await;
    assert_eq!(unknown_connection.status, StatusCode::NOT_FOUND);
    let with_api_key = app
        .send(
            "POST",
            "/v1/events/trigger",
            Some(API_KEY),
            Some(json!({ "type": "connection.active", "connected_account_id": id })),
            &[],
        )
        .await;
    assert_eq!(with_api_key.status, StatusCode::UNAUTHORIZED);
}
