//! `/health` on the API: the database and the build's commit.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use crate::testing::setup_test_context;
use axum::http::StatusCode;
use sqlx::PgPool;

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn answers_health_with_the_commit_and_503_once_the_database_is_down(pool: PgPool) {
    let app = setup_test_context(pool.clone()).await;
    let up = app.send("GET", "/health", None, None, &[]).await;
    assert_eq!(up.status, StatusCode::OK);
    assert_eq!(up.body["database"], true);
    assert!(up.body["commit"].is_string());
    assert_eq!(app.send_to_worker_get("/health").await, StatusCode::OK);

    pool.close().await;
    let down = app.send("GET", "/health", None, None, &[]).await;
    assert_eq!(down.status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(down.body["database"], false);
    assert_eq!(
        app.send_to_worker_get("/health").await,
        StatusCode::SERVICE_UNAVAILABLE
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn echoes_the_callers_request_id_or_makes_one(pool: PgPool) {
    let app = setup_test_context(pool).await;

    let given = app
        .send(
            "GET",
            "/health",
            None,
            None,
            &[("x-request-id", "caller-1")],
        )
        .await;
    let first = app.send("GET", "/health", None, None, &[]).await;
    let second = app.send("GET", "/health", None, None, &[]).await;

    assert_eq!(given.headers["x-request-id"], "caller-1");
    let made = first.headers["x-request-id"].to_str().unwrap();
    assert!(!made.is_empty());
    assert_ne!(
        first.headers["x-request-id"],
        second.headers["x-request-id"]
    );
}
