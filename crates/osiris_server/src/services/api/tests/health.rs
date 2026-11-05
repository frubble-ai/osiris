//! `/health` on the API: the database and the build's commit.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use crate::testing::setup_test_context;
use axum::http::StatusCode;
use sqlx::PgPool;

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
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
