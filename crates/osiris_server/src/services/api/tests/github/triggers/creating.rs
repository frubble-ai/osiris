//! Creating a trigger: a config GitHub can't watch refused before GitHub is called, and a watch
//! whose first check fails leaving no webhook behind at GitHub.

#![allow(
    clippy::unwrap_used,
    reason = "tests: a failure here is the test failing"
)]

use super::support::{HOOK, PULL_REQUEST, answer, answer_hook_creation, create};
use crate::testing::{API_KEY, setup_test_context};
use axum::http::StatusCode;
use osiris_providers::github::mocks::MockGithubNetwork;
use serde_json::json;
use sqlx::PgPool;

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_a_config_the_provider_cannot_watch(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let connection = app.connected_to_github(3600).await;

    for (trigger, config) in [
        (
            "github.on_repo_event",
            json!({ "owner": "octo", "repo": "hello", "events": ["pushes"] }),
        ),
        (
            "github.watch_pull_request",
            json!({ "owner": "octo", "repo": "hello", "number": 0 }),
        ),
        ("github.on_nothing", json!({})),
    ] {
        let refused = create(&app, &connection, trigger, config).await;
        assert_eq!(
            refused.status,
            StatusCode::BAD_REQUEST,
            "{trigger}: {}",
            refused.body
        );
        assert_eq!(refused.body["error"]["code"], "OS_02");
    }
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn deletes_the_webhook_again_when_the_first_check_fails(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let connection = app.connected_to_github(3600).await;
    answer_hook_creation(&app).await;
    answer(
        &app,
        "GET",
        PULL_REQUEST,
        404,
        json!({ "message": "Not Found" }),
        1,
    )
    .await;
    MockGithubNetwork::on(&app.provider)
        .answer_empty("DELETE", HOOK, 204)
        .await;

    let refused = create(
        &app,
        &connection,
        "github.watch_pull_request",
        json!({ "owner": "octo", "repo": "hello", "number": 7 }),
    )
    .await;

    assert_eq!(
        refused.status,
        StatusCode::FAILED_DEPENDENCY,
        "{}",
        refused.body
    );
    let listed = app
        .send(
            "GET",
            &format!("/v1/triggers?connected_account_id={connection}"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(listed.status, StatusCode::OK, "{}", listed.body);
    assert_eq!(listed.body["data"], json!([]));
    assert!(
        app.tasks
            .pending()
            .iter()
            .all(|task| !task.name.starts_with("trigger-poll-")),
        "no check is scheduled for a trigger that was never created"
    );
}
