//! Auth configs through the router: managing them, and the errors a client gets.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use crate::testing::db_fixtures::generate_secret;
use crate::testing::generate_connect_attempt::generate_connect_attempt;
use crate::testing::{ADMIN_KEY, API_KEY, setup_test_context};
use axum::http::StatusCode;
use osiris_providers::mocks::FAKE_ID;
use serde_json::{Value, json};
use sqlx::PgPool;

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn manages_auth_configs_without_ever_returning_the_secret(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.fake_config().await;

    let read = app
        .send(
            "GET",
            &format!("/v1/auth-configs/{id}"),
            Some(ADMIN_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(read.status, StatusCode::OK);
    assert_eq!(read.body["provider"], FAKE_ID);
    assert!(read.body.get("client_secret").is_none());

    let client_id = generate_secret("client");
    let updated = app
        .send(
            "PATCH",
            &format!("/v1/auth-configs/{id}"),
            Some(ADMIN_KEY),
            Some(json!({ "client_id": client_id, "scopes": ["repo"] })),
            &[],
        )
        .await;
    assert_eq!(updated.status, StatusCode::OK, "{}", updated.body);
    assert_eq!(updated.body["client_id"], client_id);
    assert_eq!(updated.body["scopes"], json!(["repo"]));

    let listed = app
        .send("GET", "/v1/auth-configs", Some(ADMIN_KEY), None, &[])
        .await;
    assert_eq!(listed.body["data"].as_array().unwrap().len(), 1);

    let deleted = app
        .send(
            "DELETE",
            &format!("/v1/auth-configs/{id}"),
            Some(ADMIN_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(deleted.status, StatusCode::OK);
    assert_eq!(deleted.body["deleted"], true);
    let gone = app
        .send(
            "GET",
            &format!("/v1/auth-configs/{id}"),
            Some(ADMIN_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(gone.status, StatusCode::NOT_FOUND);
    assert_eq!(gone.body["error"]["code"], "OS_01");
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_to_delete_an_auth_config_a_connection_uses(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let config = app.fake_config().await;
    app.start_connect(&config).await;
    let answer = app
        .send(
            "DELETE",
            &format!("/v1/auth-configs/{config}"),
            Some(ADMIN_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(answer.status, StatusCode::CONFLICT, "{}", answer.body);
    assert_eq!(answer.body["error"]["code"], "OS_03");
}

async fn answers_an_invalid_request_with_os_02(
    pool: PgPool,
    route: &str,
    key: &str,
    body: impl FnOnce(&str) -> Value,
) {
    let app = setup_test_context(pool).await;
    let config = app.fake_config().await;

    let answer = app
        .send("POST", route, Some(key), Some(body(&config)), &[])
        .await;

    assert_eq!(answer.status, StatusCode::BAD_REQUEST, "{}", answer.body);
    assert_eq!(answer.body["error"]["code"], "OS_02");
    assert!(answer.body["error"]["message"].is_string());
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_an_unknown_provider(pool: PgPool) {
    answers_an_invalid_request_with_os_02(
        pool,
        "/v1/auth-configs",
        ADMIN_KEY,
        |_| json!({ "provider": "nope", "client_id": "c", "client_secret": "s" }),
    )
    .await;
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_an_empty_client_secret(pool: PgPool) {
    answers_an_invalid_request_with_os_02(
        pool,
        "/v1/auth-configs",
        ADMIN_KEY,
        |_| json!({ "provider": FAKE_ID, "client_id": "c", "client_secret": "" }),
    )
    .await;
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_a_blank_client_id_on_update(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.fake_config().await;
    let answer = app
        .send(
            "PATCH",
            &format!("/v1/auth-configs/{id}"),
            Some(ADMIN_KEY),
            Some(json!({ "client_id": " " })),
            &[],
        )
        .await;
    assert_eq!(answer.status, StatusCode::BAD_REQUEST, "{}", answer.body);
    assert_eq!(answer.body["error"]["code"], "OS_02");
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_a_blank_client_secret_on_update(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.fake_config().await;
    let answer = app
        .send(
            "PATCH",
            &format!("/v1/auth-configs/{id}"),
            Some(ADMIN_KEY),
            Some(json!({ "client_secret": " " })),
            &[],
        )
        .await;
    assert_eq!(answer.status, StatusCode::BAD_REQUEST, "{}", answer.body);
    assert_eq!(answer.body["error"]["code"], "OS_02");
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_a_callback_that_is_not_http(pool: PgPool) {
    answers_an_invalid_request_with_os_02(pool, "/v1/connected-accounts", API_KEY, |config| {
        let mut attempt = generate_connect_attempt(config);
        attempt["callback_url"] = "ftp://app.test/cb".into();
        attempt
    })
    .await;
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_a_blank_user(pool: PgPool) {
    answers_an_invalid_request_with_os_02(pool, "/v1/connected-accounts", API_KEY, |config| {
        json!({ "auth_config_id": config, "user_id": " ", "callback_url": "http://app.test/cb" })
    })
    .await;
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_settings_a_provider_does_not_read(pool: PgPool) {
    answers_an_invalid_request_with_os_02(pool, "/v1/auth-configs", ADMIN_KEY, |_| {
        json!({
            "provider": FAKE_ID,
            "client_id": "c",
            "client_secret": "s",
            "settings": { "anything": true },
            "secret_settings": {},
        })
    })
    .await;
}
