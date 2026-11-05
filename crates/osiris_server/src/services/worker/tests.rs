//! The worker's HTTP concerns: who may send a task, a body that is not a task, how a task's result
//! is answered, and `/health` while draining.

#![allow(
    clippy::unwrap_used,
    reason = "tests: a failure here is the test failing"
)]

use crate::testing::{WORKER_TOKEN, setup_test_context};
use axum::http::StatusCode;
use osiris_providers::github::mocks::MockGithubNetwork;
use osiris_tasks::names;
use sqlx::PgPool;

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn checks_the_worker_token_and_acknowledges_a_body_that_is_not_a_task(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected(28800).await;
    let task = app.tasks.get(&names::refresh(&id, 1)).unwrap();
    let body = serde_json::to_vec(&task).unwrap();

    assert_eq!(
        app.send_to_worker(body.clone(), None).await,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        app.send_to_worker(body, Some("wrong")).await,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        app.send_to_worker(b"not a task".to_vec(), Some(WORKER_TOKEN))
            .await,
        StatusCode::NO_CONTENT,
        "sending it again would never help"
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn answers_the_worker_s_health_with_503_once_it_is_draining(pool: PgPool) {
    let app = setup_test_context(pool).await;
    assert_eq!(app.send_to_worker_get("/health").await, StatusCode::OK);
    app.draining
        .store(true, std::sync::atomic::Ordering::SeqCst);
    assert_eq!(
        app.send_to_worker_get("/health").await,
        StatusCode::SERVICE_UNAVAILABLE
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn answers_204_for_a_task_that_ran(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected(28800).await;
    MockGithubNetwork::on(&app.provider)
        .answer_refresh(
            200,
            serde_json::json!({ "access_token": "ghu_2", "refresh_token": "ghr_2", "expires_in": 28800 }),
        )
        .await;

    assert_eq!(
        app.deliver_pending(&names::refresh(&id, 1)).await,
        StatusCode::NO_CONTENT
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn answers_500_for_a_task_that_failed_so_the_scheduler_runs_it_again(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected(28800).await;
    MockGithubNetwork::on(&app.provider)
        .answer_refresh_text(503, "Service Unavailable")
        .await;

    assert_eq!(
        app.deliver_pending(&names::refresh(&id, 1)).await,
        StatusCode::INTERNAL_SERVER_ERROR
    );
}
