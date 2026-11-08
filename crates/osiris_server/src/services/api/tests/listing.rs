//! Listing connections: the filters, cursor pages, and the refusal of an unknown status.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use crate::testing::generate_connect_attempt::generate_connect_attempt;
use crate::testing::{ADMIN_KEY, API_KEY, TestContext, setup_test_context};
use axum::http::StatusCode;
use osiris_providers::{
    REGISTRATIONS,
    mocks::fake::{FAKE_ID, MockFakeNetwork},
};
use sqlx::PgPool;

async fn ids_listed(app: &TestContext, query: &str) -> Vec<String> {
    let answer = app
        .send(
            "GET",
            &format!("/v1/connected-accounts?{query}"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(answer.status, StatusCode::OK, "{}", answer.body);
    answer.body["data"]
        .as_array()
        .unwrap()
        .iter()
        .map(|account| account["id"].as_str().unwrap().to_owned())
        .collect()
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn filters_connections_by_status_alias_provider_and_user(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let active = app.connected(28800).await;
    let config = app.fake_config().await;
    let attempt = generate_connect_attempt(&config);
    let (pending, _) = app.start_connect_with(&attempt).await;
    let alias = attempt["alias"].as_str().unwrap();
    let user = attempt["user_id"].as_str().unwrap();

    assert_eq!(
        ids_listed(&app, "status=active").await,
        std::slice::from_ref(&active)
    );
    assert_eq!(
        ids_listed(&app, "status=pending").await,
        std::slice::from_ref(&pending)
    );
    assert_eq!(
        ids_listed(&app, &format!("alias={alias}")).await,
        std::slice::from_ref(&pending)
    );
    assert_eq!(
        ids_listed(&app, &format!("user_id={user}")).await,
        std::slice::from_ref(&pending)
    );
    assert_eq!(
        ids_listed(&app, &format!("provider={FAKE_ID}")).await.len(),
        2
    );
    let other = REGISTRATIONS
        .iter()
        .map(|registration| registration.id)
        .find(|id| *id != FAKE_ID)
        .unwrap();
    assert!(
        ids_listed(&app, &format!("provider={other}"))
            .await
            .is_empty()
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_an_unknown_status_filter(pool: PgPool) {
    let app = setup_test_context(pool).await;

    let answer = app
        .send(
            "GET",
            "/v1/connected-accounts?status=nope",
            Some(API_KEY),
            None,
            &[],
        )
        .await;

    assert_eq!(answer.status, StatusCode::BAD_REQUEST);
    assert_eq!(answer.body["error"]["code"], "OS_02", "{}", answer.body);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn pages_connections_forwards_and_refuses_a_cursor_naming_a_deleted_one(pool: PgPool) {
    let app = setup_test_context(pool).await;
    for _ in 0..3 {
        app.connected(28800).await;
    }

    let first = app
        .send(
            "GET",
            "/v1/connected-accounts?limit=2",
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(first.status, StatusCode::OK, "{}", first.body);
    let first_ids: Vec<_> = first.body["data"]
        .as_array()
        .unwrap()
        .iter()
        .map(|account| account["id"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(first_ids.len(), 2);
    assert!(first.body["previous_cursor"].is_null());
    let next = first.body["next_cursor"].as_str().unwrap().to_owned();

    let second = app
        .send(
            "GET",
            &format!("/v1/connected-accounts?limit=2&after={next}"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(second.status, StatusCode::OK, "{}", second.body);
    assert_eq!(second.body["data"].as_array().unwrap().len(), 1);
    assert!(second.body["next_cursor"].is_null());
    let previous = second.body["previous_cursor"].as_str().unwrap();

    let back = app
        .send(
            "GET",
            &format!("/v1/connected-accounts?limit=2&before={previous}"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(back.status, StatusCode::OK, "{}", back.body);
    let back_ids: Vec<_> = back.body["data"]
        .as_array()
        .unwrap()
        .iter()
        .map(|account| account["id"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(back_ids, first_ids);

    MockFakeNetwork::on(&app.provider)
        .answer_revoke(401, None)
        .await;
    let deleted = app
        .send(
            "DELETE",
            &format!("/v1/connected-accounts/{next}"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(deleted.status, StatusCode::OK, "{}", deleted.body);

    let refused = app
        .send(
            "GET",
            &format!("/v1/connected-accounts?after={next}"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(refused.status, StatusCode::BAD_REQUEST);
    assert_eq!(refused.body["error"]["code"], "OS_02", "{}", refused.body);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_a_request_it_cannot_read_as_os_02(pool: PgPool) {
    let app = setup_test_context(pool).await;

    for (path, key) in [
        ("/v1/connected-accounts?limit=abc", API_KEY),
        ("/v1/auth-configs?limit=9223372036854775808", ADMIN_KEY),
        // A cursor is an id: a NUL or a space can't be one, and must not reach the database.
        ("/v1/connected-accounts?before=%00", API_KEY),
        ("/v1/auth-configs?after=not%20an%20id", ADMIN_KEY),
        // PostgreSQL can't store a NUL as text, wherever a client puts it.
        ("/v1/connected-accounts?alias=a%00b", API_KEY),
        ("/v1/connected-accounts?user_id=%00", API_KEY),
        ("/v1/connected-accounts/ca_%00", API_KEY),
        ("/v1/auth-configs/ac_%00", ADMIN_KEY),
    ] {
        let refused = app.send("GET", path, Some(key), None, &[]).await;

        assert_eq!(
            refused.status,
            StatusCode::BAD_REQUEST,
            "{path}: {}",
            refused.body
        );
        assert_eq!(refused.body["error"]["code"], "OS_02", "{path}");
    }
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_a_nul_in_a_body_s_field_as_os_02(pool: PgPool) {
    let app = setup_test_context(pool).await;

    let refused = app
        .send(
            "POST",
            "/v1/auth-configs",
            Some(ADMIN_KEY),
            Some(serde_json::json!({ "provider": FAKE_ID, "client_id": "a\u{0}b", "client_secret": "s" })),
            &[],
        )
        .await;

    assert_eq!(refused.status, StatusCode::BAD_REQUEST, "{}", refused.body);
    assert_eq!(refused.body["error"]["code"], "OS_02");
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn reads_an_empty_cursor_as_no_cursor(pool: PgPool) {
    let app = setup_test_context(pool).await;

    for (path, key) in [
        ("/v1/connected-accounts?after=", API_KEY),
        ("/v1/auth-configs?before=", ADMIN_KEY),
    ] {
        let answer = app.send("GET", path, Some(key), None, &[]).await;

        assert_eq!(answer.status, StatusCode::OK, "{path}: {}", answer.body);
    }
}
