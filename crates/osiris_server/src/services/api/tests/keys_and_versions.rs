//! The keys and the version header, before any route runs.

use crate::testing::{ADMIN_KEY, API_KEY, setup_test_context};
use axum::http::StatusCode;
use osiris_providers::mocks::FAKE_ID;
use serde_json::{Value, json};
use sqlx::PgPool;

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn keeps_each_resource_behind_its_own_key(pool: PgPool) {
    let app = setup_test_context(pool).await;
    assert_eq!(
        app.send("GET", "/v1/auth-configs", None, None, &[])
            .await
            .status,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        app.send("GET", "/v1/auth-configs", Some(API_KEY), None, &[])
            .await
            .status,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        app.send("GET", "/v1/auth-configs", Some(ADMIN_KEY), None, &[])
            .await
            .status,
        StatusCode::OK
    );
    assert_eq!(
        app.send("GET", "/v1/connected-accounts", Some(ADMIN_KEY), None, &[])
            .await
            .status,
        StatusCode::UNAUTHORIZED
    );
    let refused = app
        .send("GET", "/v1/connected-accounts", Some("wrong"), None, &[])
        .await;
    assert_eq!(refused.body["error"]["code"], "OS_00");
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn serves_the_pinned_version_and_refuses_an_unknown_one(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let pinned = app
        .send("GET", "/v1/auth-configs", Some(ADMIN_KEY), None, &[])
        .await;
    assert_eq!(pinned.headers["osiris-version"], "2026-11-04.anubis");
    let unknown = app
        .send(
            "GET",
            "/v1/auth-configs",
            Some(ADMIN_KEY),
            None,
            &[("osiris-version", "2020-01-01.ra")],
        )
        .await;
    assert_eq!(unknown.status, StatusCode::BAD_REQUEST);
    assert_eq!(unknown.body["error"]["code"], "OS_07");
}

async fn keeps_an_auth_config_change_behind_the_admin_key(
    pool: PgPool,
    method: &str,
    path: &str,
    body: Option<Value>,
) {
    let app = setup_test_context(pool).await;
    let config = app.fake_config().await;
    let path = path.replace("{config}", &config);
    let before = app
        .send(
            "GET",
            &format!("/v1/auth-configs/{config}"),
            Some(ADMIN_KEY),
            None,
            &[],
        )
        .await;

    let refused = app.send(method, &path, Some(API_KEY), body, &[]).await;

    assert_eq!(refused.status, StatusCode::UNAUTHORIZED);
    assert_eq!(refused.body["error"]["code"], "OS_00");
    // Nothing changed: the app is still there, unchanged.
    let read = app
        .send(
            "GET",
            &format!("/v1/auth-configs/{config}"),
            Some(ADMIN_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(read.status, StatusCode::OK);
    assert_eq!(read.body, before.body);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn keeps_creating_an_auth_config_behind_the_admin_key(pool: PgPool) {
    keeps_an_auth_config_change_behind_the_admin_key(
        pool,
        "POST",
        "/v1/auth-configs",
        Some(json!({ "provider": FAKE_ID, "client_id": "c", "client_secret": "s" })),
    )
    .await;
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn keeps_updating_an_auth_config_behind_the_admin_key(pool: PgPool) {
    keeps_an_auth_config_change_behind_the_admin_key(
        pool,
        "PATCH",
        "/v1/auth-configs/{config}",
        Some(json!({ "client_id": "other" })),
    )
    .await;
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn keeps_deleting_an_auth_config_behind_the_admin_key(pool: PgPool) {
    keeps_an_auth_config_change_behind_the_admin_key(
        pool,
        "DELETE",
        "/v1/auth-configs/{config}",
        None,
    )
    .await;
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn keeps_the_triggers_behind_the_api_key(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let body = json!({ "connected_account_id": "ca_missing", "trigger": "x.y", "config": {} });
    for (method, path, body) in [
        ("GET", "/v1/triggers", None),
        ("POST", "/v1/triggers", Some(body)),
        ("GET", "/v1/triggers/tr_missing", None),
        ("DELETE", "/v1/triggers/tr_missing", None),
    ] {
        for key in [None, Some(ADMIN_KEY), Some("wrong")] {
            let refused = app.send(method, path, key, body.clone(), &[]).await;
            assert_eq!(
                refused.status,
                StatusCode::UNAUTHORIZED,
                "{method} {path} with {key:?}: {}",
                refused.body
            );
            assert_eq!(refused.body["error"]["code"], "OS_00");
        }
    }
    assert_eq!(
        app.send("GET", "/v1/triggers", Some(API_KEY), None, &[])
            .await
            .status,
        StatusCode::OK
    );
}
