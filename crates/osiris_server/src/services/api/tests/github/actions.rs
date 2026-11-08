//! GitHub's actions through the router: typed reads, a refusal whose text stays GitHub's, and a
//! token GitHub rejects for good.

use crate::testing::{API_KEY, setup_test_context};
use axum::http::StatusCode;
use osiris_domain::EventType;
use osiris_providers::{
    github::mocks::{
        MockGithubNetwork, generate_github_installation::generate_github_installation,
        generate_github_repository::generate_github_repository,
    },
    mocks::generate_token_answer::generate_token_answer,
};
use sqlx::PgPool;

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn reads_installations_with_the_connection_s_token(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let tokens = generate_token_answer(Some(28800));
    let id = app.connected_to_github_with(tokens.clone()).await;
    let installation = generate_github_installation(7, false);
    MockGithubNetwork::on(&app.provider)
        .answer_installations_for_token(
            tokens["access_token"].as_str().unwrap(),
            std::slice::from_ref(&installation),
        )
        .await;

    let answer = app
        .send(
            "POST",
            &format!("/v1/connected-accounts/{id}/actions/github.list_installations"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(answer.status, StatusCode::OK, "{}", answer.body);
    assert_eq!(
        answer.body["data"][0]["account"],
        installation["account"]["login"]
    );
    assert_eq!(answer.body["data"][0]["account_type"], "user");
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn revokes_a_connection_whose_token_stays_rejected_after_a_refresh(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected_to_github(28800).await;
    let github = MockGithubNetwork::on(&app.provider);
    github.fail_installations(401, "Bad credentials").await;
    // The refresh works, but the provider still rejects the new token: the grant is gone.
    github
        .answer_refresh(200, generate_token_answer(Some(28800)))
        .await;

    let answer = app
        .send(
            "POST",
            &format!("/v1/connected-accounts/{id}/actions/github.list_installations"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(answer.status, StatusCode::CONFLICT, "{}", answer.body);
    assert_eq!(answer.body["error"]["code"], "OS_06");
    let account = app
        .send(
            "GET",
            &format!("/v1/connected-accounts/{id}"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(account.body["status"], "expired");
    assert!(
        app.events()
            .iter()
            .any(|event| event.event_type == EventType::ConnectionRevoked),
        "a connection.revoked was published"
    );
    assert!(
        app.tasks
            .pending()
            .iter()
            .all(|task| task.payload.connected_account_id() != id),
        "a revoked connection has no refresh task"
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn lists_github_repositories_through_the_router(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected_to_github(28800).await;
    let github = MockGithubNetwork::on(&app.provider);
    github
        .answer_installations(&[generate_github_installation(1, false)])
        .await;
    let repository = generate_github_repository(9);
    github
        .answer_repositories(1, None, 1, std::slice::from_ref(&repository), None)
        .await;
    let answer = app
        .send(
            "POST",
            &format!("/v1/connected-accounts/{id}/actions/github.list_repositories"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(answer.status, StatusCode::OK, "{}", answer.body);
    assert_eq!(answer.body["data"][0]["full_name"], repository["full_name"]);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn never_returns_the_provider_s_refusal_text_to_the_client(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected_to_github(28800).await;
    MockGithubNetwork::on(&app.provider)
        .fail_installations(403, "secret internal detail")
        .await;
    let answer = app
        .send(
            "POST",
            &format!("/v1/connected-accounts/{id}/actions/github.list_installations"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(
        answer.status,
        StatusCode::FAILED_DEPENDENCY,
        "{}",
        answer.body
    );
    assert_eq!(answer.body["error"]["code"], "OS_04");
    assert!(!answer.body.to_string().contains("secret internal detail"));
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_an_action_on_a_connection_kept_without_tokens(pool: PgPool) {
    let app = setup_test_context(pool).await;
    MockGithubNetwork::on(&app.provider)
        .answer_code_exchange(generate_token_answer(Some(28800)))
        .await;
    let config = app.github_config().await;
    let (id, state) = app.start_connect(&config).await;
    // Its first refresh can't be scheduled, so the connection is disabled without its tokens.
    app.tasks.fail(true);
    let back = app.come_back(&format!("state={state}&code=c")).await;
    assert!(back.contains("status=disabled"), "{back}");

    // The user must connect again.
    let action = app
        .send(
            "POST",
            &format!("/v1/connected-accounts/{id}/actions/github.list_installations"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(action.status, StatusCode::CONFLICT, "{}", action.body);
}
