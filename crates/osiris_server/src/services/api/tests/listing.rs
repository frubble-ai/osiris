//! Listing connections: the filters, and the refusal of an unknown status.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use crate::testing::generate_connect_attempt::generate_connect_attempt;
use crate::testing::{API_KEY, TestContext, setup_test_context};
use axum::http::StatusCode;
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
    let config = app.github_config().await;
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
    assert_eq!(ids_listed(&app, "provider=github").await.len(), 2);
    assert!(ids_listed(&app, "provider=slack").await.is_empty());
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
