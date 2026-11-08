//! The triggers' public delivery route reads a body up to `[triggers] max_delivery_bytes`, its own
//! limit, apart from the API's.

#![allow(
    clippy::unwrap_used,
    reason = "tests: a failure here is the test failing"
)]

use crate::testing::setup_test_context_with;
use axum::http::StatusCode;
use sqlx::PgPool;

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
