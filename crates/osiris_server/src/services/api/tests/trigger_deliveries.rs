//! The triggers' public delivery route reads a body up to `[triggers] max_delivery_bytes`, its own
//! limit, apart from the API's, and at most `[triggers] max_concurrent_deliveries` at once.

#![allow(
    clippy::unwrap_used,
    reason = "tests: a failure here is the test failing"
)]

use crate::testing::setup_test_context_with;
use axum::{
    body::{Body, Bytes},
    http::{Request, StatusCode},
};
use sqlx::PgPool;
use std::time::Duration;
use tower::ServiceExt;

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_a_delivery_larger_than_its_limit_and_reads_one_within_it(pool: PgPool) {
    let app = setup_test_context_with(pool, |config| {
        config.settings.triggers.max_delivery_bytes = 1024;
    })
    .await;
    let deliver = |body: String| {
        let app = &app;
        async move {
            app.send_raw("POST", "/webhooks/triggers/tr_missing", None, body, &[])
                .await
                .status
        }
    };

    assert_eq!(
        deliver("x".repeat(2048)).await,
        StatusCode::PAYLOAD_TOO_LARGE
    );
    // Within the limit, the body is read: an unsigned delivery reads as a trigger that isn't there.
    assert_eq!(deliver("{}".to_owned()).await, StatusCode::NOT_FOUND);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn reads_no_more_deliveries_at_once_than_its_limit(pool: PgPool) {
    let app = setup_test_context_with(pool, |config| {
        config.settings.triggers.max_concurrent_deliveries = 1;
    })
    .await;
    let delivery = |body: Body| {
        Request::builder()
            .method("POST")
            .uri("/webhooks/triggers/tr_missing")
            .header("content-type", "application/json")
            .body(body)
            .unwrap()
    };
    // The first delivery's body arrives only when the test sends it, so it holds the one place.
    let (send, body) = futures::channel::mpsc::unbounded::<Result<Bytes, std::io::Error>>();
    let first = tokio::spawn(app.api.clone().oneshot(delivery(Body::from_stream(body))));
    tokio::task::yield_now().await;

    let second = app.api.clone().oneshot(delivery(Body::from("{}")));
    tokio::pin!(second);
    let waited = tokio::time::timeout(Duration::from_millis(300), &mut second).await;
    assert!(
        waited.is_err(),
        "the second delivery was read while the first held the limit"
    );

    send.unbounded_send(Ok(Bytes::from_static(b"{}"))).unwrap();
    drop(send);
    assert_eq!(
        first.await.unwrap().unwrap().status(),
        StatusCode::NOT_FOUND
    );
    assert_eq!(second.await.unwrap().status(), StatusCode::NOT_FOUND);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn answers_a_body_too_slow_to_arrive_408_and_lets_the_next_delivery_in(pool: PgPool) {
    let app = setup_test_context_with(pool, |config| {
        config.settings.triggers.max_concurrent_deliveries = 1;
        config.settings.triggers.delivery_read_seconds = 1;
    })
    .await;
    let delivery = |body: Body| {
        Request::builder()
            .method("POST")
            .uri("/webhooks/triggers/tr_missing")
            .header("content-type", "application/json")
            .body(body)
            .unwrap()
    };
    // A body that never arrives, its sender kept open until the end of the test.
    let (_sending, stalled) = futures::channel::mpsc::unbounded::<Result<Bytes, std::io::Error>>();

    let slow = app
        .api
        .clone()
        .oneshot(delivery(Body::from_stream(stalled)))
        .await
        .unwrap();
    let next = app
        .api
        .clone()
        .oneshot(delivery(Body::from("{}")))
        .await
        .unwrap();

    assert_eq!(slow.status(), StatusCode::REQUEST_TIMEOUT);
    assert_eq!(next.status(), StatusCode::NOT_FOUND);
}
