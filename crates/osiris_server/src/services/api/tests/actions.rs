//! Actions: typed reads, and a token the provider rejects for good.

use crate::testing::{API_KEY, setup_test_context};
use axum::http::StatusCode;
use osiris_domain::EventType;
use osiris_providers::{
    github::mocks::{
        MockGithubNetwork, generate_github_installation::generate_github_installation,
        generate_github_repository::generate_github_repository,
    },
    linear::mocks::{MockLinearNetwork, generate_linear_team::generate_linear_team},
    mocks::generate_token_answer::generate_token_answer,
    slack::mocks::{MockSlackNetwork, generate_slack_channel::generate_slack_channel},
};
use sqlx::PgPool;

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn reads_installations_with_the_connection_s_token(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let tokens = generate_token_answer(Some(28800));
    let id = app.connected_with(tokens.clone()).await;
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

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn refuses_an_action_of_another_provider(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected(28800).await;
    let answer = app
        .send(
            "POST",
            &format!("/v1/connected-accounts/{id}/actions/slack.list_channels"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(answer.status, StatusCode::BAD_REQUEST);
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn revokes_a_connection_whose_token_stays_rejected_after_a_refresh(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected(28800).await;
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

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn lists_github_repositories_through_the_router(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected(28800).await;
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

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn lists_linear_teams_through_the_router(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let (id, token) = app.connected_to("linear").await;
    let team = generate_linear_team();
    MockLinearNetwork::on(&app.provider)
        .answer_teams_for_token(&token, std::slice::from_ref(&team))
        .await;
    let answer = app
        .send(
            "POST",
            &format!("/v1/connected-accounts/{id}/actions/linear.list_teams"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(answer.status, StatusCode::OK, "{}", answer.body);
    assert_eq!(answer.body["data"][0]["name"], team["name"]);
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn lists_slack_channels_through_the_router(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let (id, _) = app.connected_to("slack").await;
    let channel = generate_slack_channel();
    MockSlackNetwork::on(&app.provider)
        .answer_channels_page(Some(""), std::slice::from_ref(&channel), "", None)
        .await;
    let answer = app
        .send(
            "POST",
            &format!("/v1/connected-accounts/{id}/actions/slack.list_channels"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(answer.status, StatusCode::OK, "{}", answer.body);
    assert_eq!(answer.body["data"][0]["name"], channel["name"]);
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn never_returns_the_provider_s_refusal_text_to_the_client(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected(28800).await;
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
