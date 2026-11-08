//! Disconnecting through the router: the grant ended at the provider, and when it can't be.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use crate::testing::{API_KEY, setup_test_context};
use axum::http::StatusCode;
use osiris_providers::mocks::{
    fake::MockFakeNetwork, generate_token_answer::generate_token_answer,
};
use sqlx::PgPool;

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn deletes_a_connection_whose_grant_the_provider_no_longer_has(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected(28800).await;
    MockFakeNetwork::on(&app.provider)
        .answer_revoke(401, None)
        .await;
    let answer = app
        .send(
            "DELETE",
            &format!("/v1/connected-accounts/{id}"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(answer.status, StatusCode::OK, "{}", answer.body);
    let gone = app
        .send(
            "GET",
            &format!("/v1/connected-accounts/{id}"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(gone.status, StatusCode::NOT_FOUND);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn keeps_a_connection_when_the_provider_cannot_end_its_grant(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected(28800).await;
    MockFakeNetwork::on(&app.provider)
        .answer_revoke(503, None)
        .await;
    let answer = app
        .send(
            "DELETE",
            &format!("/v1/connected-accounts/{id}"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(answer.status, StatusCode::BAD_GATEWAY, "{}", answer.body);
    assert_eq!(answer.body["error"]["code"], "OS_05");
    assert!(
        !answer.body["error"]["message"]
            .as_str()
            .unwrap()
            .contains("503"),
        "the provider's answer stays in the logs"
    );
    assert_eq!(app.account(&id).await["status"], "active");
    let refresh = osiris_tasks::names::refresh(&id, 1);
    assert!(
        !app.tasks.cancelled().contains(&refresh) && app.tasks.get(&refresh).is_some(),
        "its refresh stays scheduled"
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn ends_the_grant_at_the_provider_when_deleting_a_disabled_connection(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected(28800).await;
    MockFakeNetwork::on(&app.provider)
        .answer_refresh(200, generate_token_answer(Some(28800)))
        .await;
    let task = app
        .tasks
        .take(&osiris_tasks::names::refresh(&id, 1))
        .unwrap();
    app.tasks.fail(true);
    app.deliver(&task).await;
    app.tasks.fail(false);
    assert_eq!(app.account(&id).await["status"], "disabled");
    MockFakeNetwork::on(&app.provider)
        .answer_revoke(200, Some(1))
        .await;

    let answer = app
        .send(
            "DELETE",
            &format!("/v1/connected-accounts/{id}"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(answer.status, StatusCode::OK, "{}", answer.body);
}
