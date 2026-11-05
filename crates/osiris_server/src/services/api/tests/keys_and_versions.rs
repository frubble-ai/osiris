//! The keys and the version header, before any route runs.

use crate::testing::{ADMIN_KEY, API_KEY, setup_test_context};
use axum::http::StatusCode;
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

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn keeps_every_auth_config_change_behind_the_admin_key(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let config = app.github_config().await;
    let changes = [
        (
            "POST",
            "/v1/auth-configs".to_owned(),
            Some(
                serde_json::json!({ "provider": "github", "client_id": "c", "client_secret": "s" }),
            ),
        ),
        (
            "PATCH",
            format!("/v1/auth-configs/{config}"),
            Some(serde_json::json!({ "client_id": "other" })),
        ),
        ("DELETE", format!("/v1/auth-configs/{config}"), None),
    ];
    for (method, path, body) in changes {
        let refused = app
            .send(method, &path, Some(API_KEY), body.clone(), &[])
            .await;
        assert_eq!(refused.status, StatusCode::UNAUTHORIZED, "{method} {path}");
        assert_eq!(refused.body["error"]["code"], "OS_00", "{method} {path}");
    }
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
    assert_eq!(read.body["client_id"], "client");
}
