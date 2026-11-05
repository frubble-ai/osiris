//! Auth configs through the router: managing them, and the errors a client gets.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use crate::testing::{ADMIN_KEY, API_KEY, setup_test_context};
use axum::http::StatusCode;
use serde_json::json;
use sqlx::PgPool;

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn manages_auth_configs_without_ever_returning_the_secret(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.github_config().await;

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
    assert_eq!(read.body["provider"], "github");
    assert!(read.body.get("client_secret").is_none());

    let updated = app
        .send(
            "PATCH",
            &format!("/v1/auth-configs/{id}"),
            Some(ADMIN_KEY),
            Some(json!({ "client_id": "Iv1.other", "scopes": ["repo"] })),
            &[],
        )
        .await;
    assert_eq!(updated.status, StatusCode::OK, "{}", updated.body);
    assert_eq!(updated.body["client_id"], "Iv1.other");
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

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn refuses_to_delete_an_auth_config_a_connection_uses(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let config = app.github_config().await;
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

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn answers_invalid_requests_with_os_02(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let config = app.github_config().await;
    let invalid = [
        (
            "/v1/auth-configs",
            ADMIN_KEY,
            json!({ "provider": "nope", "client_id": "c", "client_secret": "s" }),
        ),
        (
            "/v1/auth-configs",
            ADMIN_KEY,
            json!({ "provider": "github", "client_id": "c", "client_secret": "" }),
        ),
        (
            "/v1/connected-accounts",
            API_KEY,
            json!({ "auth_config_id": config, "user_id": "org_1", "callback_url": "ftp://app.test/cb" }),
        ),
        (
            "/v1/connected-accounts",
            API_KEY,
            json!({ "auth_config_id": config, "user_id": " ", "callback_url": "http://app.test/cb" }),
        ),
    ];
    for (route, key, body) in invalid {
        let answer = app
            .send("POST", route, Some(key), Some(body.clone()), &[])
            .await;
        assert_eq!(
            answer.status,
            StatusCode::BAD_REQUEST,
            "{body}: {}",
            answer.body
        );
        assert_eq!(answer.body["error"]["code"], "OS_02", "{body}");
        assert!(answer.body["error"]["message"].is_string());
    }
}
