//! Slack's actions through the router: an action runs with the connection's token, one whose scope
//! the connection wasn't granted is refused before Slack is called, and a connection to another
//! provider can't run them.

#![allow(
    clippy::unwrap_used,
    reason = "tests: a failure here is the test failing"
)]

use crate::testing::{API_KEY, setup_test_context};
use axum::http::StatusCode;
use osiris_providers::slack::mocks::{
    MockSlackNetwork, generate_slack_channel::generate_slack_channel,
};
use sqlx::PgPool;

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn lists_slack_channels_through_the_router(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let (id, _) = app.connected_to_slack().await;
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

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_listing_channels_on_a_connection_not_granted_channels_read(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let (id, _) = app.connected_to_slack_granting("users:read").await;

    let answer = app
        .send(
            "POST",
            &format!("/v1/connected-accounts/{id}/actions/slack.list_channels"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;

    // Slack was never asked: the faked network has nothing arranged for the channels.
    assert_eq!(answer.status, StatusCode::FORBIDDEN, "{}", answer.body);
    assert_eq!(answer.body["error"]["code"], "OS_11");
    assert!(
        answer.body["error"]["message"]
            .as_str()
            .unwrap()
            .contains("channels:read")
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_a_connection_to_another_provider(pool: PgPool) {
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
    assert_eq!(answer.status, StatusCode::BAD_REQUEST, "{}", answer.body);
}
