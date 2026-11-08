//! Refreshing a connection now: what is refused, and how.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use crate::testing::{ADMIN_KEY, setup_test_context};
use axum::http::StatusCode;
use osiris_providers::mocks::{
    fake::MockFakeNetwork, generate_token_answer::generate_token_answer_without_refresh,
};
use sqlx::PgPool;

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_to_refresh_an_unknown_connection(pool: PgPool) {
    let app = setup_test_context(pool).await;

    let answer = app
        .send(
            "POST",
            "/v1/connected-accounts/ca_unknown/refresh",
            Some(ADMIN_KEY),
            None,
            &[],
        )
        .await;

    assert_eq!(answer.status, StatusCode::NOT_FOUND, "{}", answer.body);
    assert_eq!(answer.body["error"]["code"], "OS_01");
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_to_refresh_a_connection_that_is_not_complete(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let config = app.fake_config().await;
    let (pending, _) = app.start_connect(&config).await;

    let answer = app
        .send(
            "POST",
            &format!("/v1/connected-accounts/{pending}/refresh"),
            Some(ADMIN_KEY),
            None,
            &[],
        )
        .await;

    assert_eq!(answer.status, StatusCode::CONFLICT, "{}", answer.body);
    assert_eq!(answer.body["error"]["code"], "OS_03");
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_to_refresh_a_disabled_connection_whose_provider_sent_no_refresh_token(
    pool: PgPool,
) {
    let app = setup_test_context(pool).await;
    MockFakeNetwork::on(&app.provider)
        .answer_code_exchange(generate_token_answer_without_refresh(None))
        .await;
    let config = app.fake_config().await;
    let (id, state) = app.start_connect(&config).await;
    let back = app.come_back(&format!("state={state}&code=c")).await;
    assert!(back.contains("status=disabled"), "{back}");

    let answer = app
        .send(
            "POST",
            &format!("/v1/connected-accounts/{id}/refresh"),
            Some(ADMIN_KEY),
            None,
            &[],
        )
        .await;

    assert_eq!(answer.status, StatusCode::CONFLICT, "{}", answer.body);
    assert_eq!(answer.body["error"]["code"], "OS_03");
}
