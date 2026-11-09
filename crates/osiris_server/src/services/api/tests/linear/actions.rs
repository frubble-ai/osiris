//! Linear's actions through the router: an action runs with the connection's token, and one whose
//! scope the connection wasn't granted is refused before Linear is called.

#![allow(
    clippy::unwrap_used,
    reason = "tests: a failure here is the test failing"
)]

use crate::testing::{API_KEY, setup_test_context};
use axum::http::StatusCode;
use osiris_providers::linear::mocks::{
    MockLinearNetwork, generate_linear_team::generate_linear_team,
};
use sqlx::PgPool;

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn lists_linear_teams_through_the_router(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let (id, token) = app.connected_to_linear().await;
    let team = generate_linear_team();
    MockLinearNetwork::on(&app.provider)
        .answer_teams_for_token(&token, std::slice::from_ref(&team))
        .await;
    let answer = app
        .send(
            "POST",
            &format!("/v1/connected-accounts/{id}/actions/linear.list_teams"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(answer.status, StatusCode::OK, "{}", answer.body);
    assert_eq!(answer.body["data"][0]["name"], team["name"]);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_listing_teams_on_a_connection_not_granted_read(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let (id, _) = app.connected_to_linear_granting("write").await;

    let answer = app
        .send(
            "POST",
            &format!("/v1/connected-accounts/{id}/actions/linear.list_teams"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;

    // Linear was never asked: the faked network has nothing arranged for the teams.
    assert_eq!(answer.status, StatusCode::FORBIDDEN, "{}", answer.body);
    assert_eq!(answer.body["error"]["code"], "OS_11");
    assert!(
        answer.body["error"]["message"]
            .as_str()
            .unwrap()
            .contains("read")
    );
}
