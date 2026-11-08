//! Running an action through a connection, against the fake provider's `whoami`: a rejected token
//! refreshed and the action run again, rejected again and the connection revoked, and the
//! refusals made before the provider is called (another provider's connection, missing scopes, a
//! user's action on an installation connection).

#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "tests: a failure here is the test failing"
)]

use super::whoami::{PLAIN, Whoami, run_whoami};
use crate::testing::{TestContext, setup_test_context};
use crate::use_cases::lib::connected_accounts::create_installation::NewInstallation;
use osiris_domain::{DomainError, EventType};
use osiris_providers::mocks::{MockFakeNetwork, generate_token_answer::generate_token_answer};
use serde_json::{Value, json};
use sqlx::PgPool;

/// How often the provider was asked `whoami`.
async fn whoami_calls(app: &TestContext) -> usize {
    app.provider
        .received_requests()
        .await
        .unwrap_or_default()
        .iter()
        .filter(|request| request.url.path() == "/fake/me")
        .count()
}

/// The access token of a token answer.
fn access_token(answer: &Value) -> String {
    answer["access_token"].as_str().unwrap().to_owned()
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn answers_the_provider_s_answer_with_the_connection_s_token(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let answer = generate_token_answer(Some(28800));
    let id = app.connected_with(answer.clone()).await;
    MockFakeNetwork::on(&app.provider)
        .answer_whoami_for_token(&access_token(&answer), json!({ "login": "someone" }))
        .await;

    let who = run_whoami(&app, &id, PLAIN).await.unwrap();

    assert_eq!(who, json!({ "login": "someone" }));
    assert_eq!(whoami_calls(&app).await, 1);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refreshes_a_rejected_token_and_runs_the_action_again_with_the_new_one(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected(28800).await;
    let refreshed = generate_token_answer(Some(28800));
    let fake = MockFakeNetwork::on(&app.provider);
    fake.answer_refresh(200, refreshed.clone()).await;
    // Only the refreshed token is accepted; the first one is rejected.
    fake.answer_whoami_for_token(&access_token(&refreshed), json!({ "login": "someone" }))
        .await;
    fake.fail_whoami(401, "bad credentials").await;

    let who = run_whoami(&app, &id, PLAIN).await.unwrap();

    assert_eq!(who, json!({ "login": "someone" }));
    assert_eq!(whoami_calls(&app).await, 2);
    let account = app.account(&id).await;
    assert_eq!(account["status"], "active");
    assert_eq!(
        app.use_cases
            .context()
            .db
            .connected_account(&id)
            .await
            .unwrap()
            .token_generation,
        2
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn revokes_the_connection_when_the_refreshed_token_is_rejected_too(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected(28800).await;
    let fake = MockFakeNetwork::on(&app.provider);
    fake.answer_refresh(200, generate_token_answer(Some(28800)))
        .await;
    fake.fail_whoami(401, "bad credentials").await;

    let refused = run_whoami(&app, &id, PLAIN).await.unwrap_err();

    assert!(matches!(
        refused.current_context(),
        DomainError::NotActive("expired")
    ));
    assert_eq!(whoami_calls(&app).await, 2, "tried once more, no further");
    assert_eq!(app.account(&id).await["status"], "expired");
    assert!(
        app.events()
            .iter()
            .any(|event| event.event_type == EventType::ConnectionRevoked)
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn answers_another_failure_of_the_provider_without_refreshing(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected(28800).await;
    let fake = MockFakeNetwork::on(&app.provider);
    fake.expect_no_refresh().await;
    fake.fail_whoami(503, "down for maintenance").await;

    let failed = run_whoami(&app, &id, PLAIN).await.unwrap_err();

    assert!(matches!(
        failed.current_context(),
        DomainError::ProviderUnavailable(_)
    ));
    assert_eq!(whoami_calls(&app).await, 1);
    assert_eq!(app.account(&id).await["status"], "active");
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_a_connection_to_another_provider_before_calling_it(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected(28800).await;

    let refused = run_whoami(
        &app,
        &id,
        Whoami {
            provider: "acme",
            ..PLAIN
        },
    )
    .await
    .unwrap_err();

    assert!(matches!(refused.current_context(), DomainError::Invalid(_)));
    assert_eq!(whoami_calls(&app).await, 0);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_a_connection_lacking_the_action_s_scopes_before_calling_it(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let mut answer = generate_token_answer(Some(28800));
    answer["scope"] = "read".into();
    let id = app.connected_with(answer).await;

    let refused = run_whoami(
        &app,
        &id,
        Whoami {
            required_scopes: &["read", "write"],
            ..PLAIN
        },
    )
    .await
    .unwrap_err();

    let DomainError::MissingScopes(message) = refused.current_context() else {
        panic!("missing scopes, not {refused:?}");
    };
    assert!(message.contains("needs write;"), "{message}");
    assert_eq!(whoami_calls(&app).await, 0);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_a_user_s_action_on_an_installation_connection_before_calling_it(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let db = &app.use_cases.context().db;
    let config = crate::testing::db_fixtures::fake_config(db).await;
    let id = db
        .create_installation(NewInstallation {
            auth_config_id: config,
            user_id: "org_1".into(),
            alias: None,
            installation_id: "77".into(),
        })
        .await
        .unwrap()
        .id;

    let refused = run_whoami(
        &app,
        &id,
        Whoami {
            user_only: true,
            ..PLAIN
        },
    )
    .await
    .unwrap_err();

    assert!(matches!(refused.current_context(), DomainError::Invalid(_)));
    assert_eq!(whoami_calls(&app).await, 0);
}
