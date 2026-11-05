//! The keys and the version header, before any route runs.

#[path = "support/app.rs"]
mod support;

use axum::http::StatusCode;
use sqlx::PgPool;
use support::{ADMIN_KEY, API_KEY, app};

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn keeps_each_resource_behind_its_own_key(pool: PgPool) {
    let app = app(pool).await;
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

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn serves_the_pinned_version_and_refuses_an_unknown_one(pool: PgPool) {
    let app = app(pool).await;
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
