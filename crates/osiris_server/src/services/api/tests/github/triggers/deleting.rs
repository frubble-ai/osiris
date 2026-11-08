//! Deleting a trigger removes its webhook at GitHub, and the trigger is deleted even when GitHub
//! fails to remove it; deleting the connection removes its triggers' webhooks first, and marks
//! the triggers failed if the connection is kept because GitHub can't be reached.

#![allow(
    clippy::unwrap_used,
    reason = "tests: a failure here is the test failing"
)]

use super::support::{get_trigger, watching_repo_events};
use crate::testing::{API_KEY, TestContext, requests::Answer, setup_test_context};
use axum::http::StatusCode;
use osiris_providers::github::mocks::MockGithubNetwork;
use sqlx::PgPool;

async fn delete_trigger(app: &TestContext, id: &str) -> Answer {
    app.send(
        "DELETE",
        &format!("/v1/triggers/{id}"),
        Some(API_KEY),
        None,
        &[],
    )
    .await
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn removes_the_webhook_when_the_trigger_is_deleted(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let (_, id, watched) = watching_repo_events(&app).await;
    MockGithubNetwork::on(&app.provider)
        .answer_empty("DELETE", &watched.hook(), 204)
        .await;

    let deleted = delete_trigger(&app, &id).await;

    assert_eq!(deleted.status, StatusCode::OK, "{}", deleted.body);
    assert_eq!(deleted.body["deleted"], true);
    assert_eq!(get_trigger(&app, &id).await.status, StatusCode::NOT_FOUND);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn deletes_the_trigger_even_when_github_fails_to_remove_its_webhook(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let (_, id, watched) = watching_repo_events(&app).await;
    MockGithubNetwork::on(&app.provider)
        .answer_empty("DELETE", &watched.hook(), 500)
        .await;

    let deleted = delete_trigger(&app, &id).await;

    assert_eq!(deleted.status, StatusCode::OK, "{}", deleted.body);
    assert_eq!(deleted.body["deleted"], true);
    assert_eq!(get_trigger(&app, &id).await.status, StatusCode::NOT_FOUND);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn removes_the_triggers_webhooks_when_the_connection_is_deleted(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let (connection, id, watched) = watching_repo_events(&app).await;
    let github = MockGithubNetwork::on(&app.provider);
    github.answer_empty("DELETE", &watched.hook(), 204).await;
    github.answer_revoke(204, Some(1)).await;

    let deleted = app
        .send(
            "DELETE",
            &format!("/v1/connected-accounts/{connection}"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;

    assert_eq!(deleted.status, StatusCode::OK, "{}", deleted.body);
    assert_eq!(get_trigger(&app, &id).await.status, StatusCode::NOT_FOUND);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn marks_the_triggers_failed_when_the_connection_is_kept_because_github_is_down(
    pool: PgPool,
) {
    let app = setup_test_context(pool).await;
    let (connection, id, watched) = watching_repo_events(&app).await;
    let github = MockGithubNetwork::on(&app.provider);
    github.answer_empty("DELETE", &watched.hook(), 204).await;
    github.answer_revoke(503, None).await;

    let kept = app
        .send(
            "DELETE",
            &format!("/v1/connected-accounts/{connection}"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;

    assert_ne!(kept.status, StatusCode::OK, "{}", kept.body);
    assert_eq!(app.account(&connection).await["status"], "active");
    let trigger = get_trigger(&app, &id).await;
    assert_eq!(trigger.status, StatusCode::OK, "{}", trigger.body);
    assert_eq!(trigger.body["status"], "failed");
    assert!(trigger.body["last_error"].is_string());
}
