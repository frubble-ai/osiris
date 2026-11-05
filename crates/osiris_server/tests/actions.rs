//! Actions: typed reads, and a token the provider rejects for good.

#[path = "support/app.rs"]
mod support;

use axum::http::StatusCode;
use osiris_domain::EventType;
use serde_json::json;
use sqlx::PgPool;
use support::{API_KEY, app};
use wiremock::{
    Mock, ResponseTemplate,
    matchers::{body_string_contains, header, method, path},
};

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn reads_installations_with_the_connection_s_token(pool: PgPool) {
    let app = app(pool).await;
    let id = app.connected(28800).await;
    Mock::given(method("GET")).and(path("/user/installations")).and(header("authorization", "Bearer ghu_1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "total_count": 1, "installations": [{ "id": 7, "account": { "login": "dan", "type": "User" }, "repository_selection": "all" }] })))
        .mount(&app.provider).await;

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
    assert_eq!(answer.body["data"][0]["account"], "dan");
    assert_eq!(answer.body["data"][0]["account_type"], "user");
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn refuses_an_action_of_another_provider(pool: PgPool) {
    let app = app(pool).await;
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
    let app = app(pool).await;
    let id = app.connected(28800).await;
    Mock::given(path("/user/installations"))
        .respond_with(ResponseTemplate::new(401).set_body_string("Bad credentials"))
        .mount(&app.provider)
        .await;
    // The refresh works, but the provider still rejects the new token: the grant is gone.
    Mock::given(method("POST"))
        .and(path("/login/oauth/access_token"))
        .and(body_string_contains("grant_type=refresh_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            json!({ "access_token": "ghu_2", "refresh_token": "ghr_2", "expires_in": 28800 }),
        ))
        .mount(&app.provider)
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
