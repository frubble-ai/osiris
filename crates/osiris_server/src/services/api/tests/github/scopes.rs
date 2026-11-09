//! GitHub's ported actions through the router: an action runs with the connection's token, and
//! one whose scopes the connection wasn't granted is refused before GitHub is called.

#![allow(
    clippy::unwrap_used,
    reason = "tests: a failure here is the test failing"
)]

use crate::testing::{API_KEY, setup_test_context};
use axum::http::StatusCode;
use osiris_providers::github::mocks::{
    MockGithubNetwork,
    generate_github_names::{generate_github_login, generate_github_repo, generate_github_token},
    generate_github_repository::generate_github_repository,
};
use serde_json::json;
use sqlx::PgPool;

fn token_answer(scope: &str) -> serde_json::Value {
    json!({
        "access_token": generate_github_token(),
        "refresh_token": generate_github_token(),
        "expires_in": 28800,
        "scope": scope,
    })
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn reads_a_repository_with_a_connection_granted_repo(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let (owner, repo) = (generate_github_login(), generate_github_repo());
    let id = app
        .connected_to_github_with(token_answer("repo,read:user"))
        .await;
    let repository = generate_github_repository(42);
    MockGithubNetwork::on(&app.provider)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}"),
            &[],
            200,
            repository.clone(),
        )
        .await;

    let answer = app
        .send(
            "POST",
            &format!("/v1/connected-accounts/{id}/actions/github.get_repository"),
            Some(API_KEY),
            Some(json!({ "owner": owner, "repo": repo })),
            &[],
        )
        .await;

    assert_eq!(answer.status, StatusCode::OK, "{}", answer.body);
    assert_eq!(answer.body["data"]["id"], "42");
    assert_eq!(answer.body["data"]["full_name"], repository["full_name"]);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_an_action_whose_scopes_the_connection_lacks(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let (owner, repo) = (generate_github_login(), generate_github_repo());
    let id = app
        .connected_to_github_with(token_answer("read:user"))
        .await;

    let answer = app
        .send(
            "POST",
            &format!("/v1/connected-accounts/{id}/actions/github.get_repository"),
            Some(API_KEY),
            Some(json!({ "owner": owner, "repo": repo })),
            &[],
        )
        .await;

    // GitHub was never asked: the faked network has nothing arranged for the repository.
    assert_eq!(answer.status, StatusCode::FORBIDDEN, "{}", answer.body);
    assert_eq!(answer.body["error"]["code"], "OS_11");
    assert!(
        answer.body["error"]["message"]
            .as_str()
            .unwrap()
            .contains("repo")
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn runs_an_action_on_a_github_app_token_with_no_scopes(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let (owner, repo) = (generate_github_login(), generate_github_repo());
    // A GitHub App's user token carries no scopes; the app's permissions decide instead.
    let id = app.connected_to_github_with(token_answer("")).await;
    MockGithubNetwork::on(&app.provider)
        .answer(
            "GET",
            &format!("/repos/{owner}/{repo}"),
            &[],
            200,
            generate_github_repository(1),
        )
        .await;

    let answer = app
        .send(
            "POST",
            &format!("/v1/connected-accounts/{id}/actions/github.get_repository"),
            Some(API_KEY),
            Some(json!({ "owner": owner, "repo": repo })),
            &[],
        )
        .await;

    assert_eq!(answer.status, StatusCode::OK, "{}", answer.body);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_an_input_with_a_field_the_action_does_not_take(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let (owner, repo) = (generate_github_login(), generate_github_repo());
    let id = app.connected_to_github_with(token_answer("repo")).await;

    let answer = app
        .send(
            "POST",
            &format!("/v1/connected-accounts/{id}/actions/github.get_repository"),
            Some(API_KEY),
            Some(json!({ "owner": owner, "repo": repo, "nope": true })),
            &[],
        )
        .await;

    assert_eq!(answer.status, StatusCode::BAD_REQUEST, "{}", answer.body);
    assert_eq!(answer.body["error"]["code"], "OS_02");
}
