//! Linear against its faked network.

#![allow(
    clippy::unwrap_used,
    reason = "tests: a failure here is the test failing"
)]

use super::{
    Linear,
    mocks::{MockLinearNetwork, generate_linear_team::generate_linear_team},
};
use crate::{ListLimits, OAuthClient, Provider, ProviderError, ProviderUrls};
use rstest::rstest;
use serde_json::json;
use url::Url;
use wiremock::MockServer;

fn linear(server: &MockServer) -> Linear {
    linear_capped_at(server, 5000)
}

fn linear_capped_at(server: &MockServer, max_items: usize) -> Linear {
    let base = Url::parse(&server.uri()).unwrap();
    Linear::new(
        &ProviderUrls {
            oauth_base: base.clone(),
            api_base: base,
        },
        ListLimits {
            concurrency: 2,
            max_items,
            max_log_bytes: 262_144,
        },
    )
    .unwrap()
}

#[tokio::test]
async fn lists_teams_across_pages() {
    let server = MockServer::start().await;
    let network = MockLinearNetwork::on(&server);
    let (first, second) = (generate_linear_team(), generate_linear_team());
    network
        .answer_teams_page(None, std::slice::from_ref(&first), Some("c1"), None)
        .await;
    network
        .answer_teams_page(Some("c1"), std::slice::from_ref(&second), None, None)
        .await;

    let teams = linear(&server)
        .list_teams(&reqwest::Client::new(), "token")
        .await
        .unwrap();

    assert_eq!(
        teams
            .iter()
            .map(|team| team.name.as_str())
            .collect::<Vec<_>>(),
        [
            first["name"].as_str().unwrap(),
            second["name"].as_str().unwrap()
        ]
    );
}

#[tokio::test]
async fn reads_an_authentication_error_beside_a_200_as_a_rejected_token() {
    let server = MockServer::start().await;
    MockLinearNetwork::on(&server)
        .answer_graphql_errors(json!([{ "message": "Authentication required", "extensions": { "code": "AUTHENTICATION_ERROR" } }]))
        .await;

    let error = linear(&server)
        .list_teams(&reqwest::Client::new(), "token")
        .await
        .unwrap_err();

    assert!(
        matches!(error, ProviderError::Unauthorized { .. }),
        "{error}"
    );
}

#[tokio::test]
async fn stops_reading_teams_at_the_cap() {
    let server = MockServer::start().await;
    let teams: Vec<_> = (0..3).map(|_| generate_linear_team()).collect();
    // Only the first page is read: a second request would fail the test.
    MockLinearNetwork::on(&server)
        .answer_teams_page(None, &teams, Some("c1"), Some(1))
        .await;

    let listed = linear_capped_at(&server, 2)
        .list_teams(&reqwest::Client::new(), "token")
        .await
        .unwrap();

    assert_eq!(listed.len(), 2);
}

#[rstest]
#[case::revoked_now(200)]
#[case::token_already_rejected(401)]
#[tokio::test]
async fn ends_a_grant_or_counts_one_linear_no_longer_accepts_as_ended(#[case] status: u16) {
    let server = MockServer::start().await;
    MockLinearNetwork::on(&server)
        .answer_revoke(status, Some(1))
        .await;

    let ended = linear(&server)
        .revoke(&reqwest::Client::new(), &client(), "token")
        .await;

    assert!(ended.is_ok());
}

#[tokio::test]
async fn keeps_the_grant_when_linear_cannot_end_it_for_now() {
    let server = MockServer::start().await;
    MockLinearNetwork::on(&server)
        .answer_revoke(503, None)
        .await;

    let failure = linear(&server)
        .revoke(&reqwest::Client::new(), &client(), "token")
        .await
        .unwrap_err();

    assert!(
        matches!(failure, ProviderError::Unavailable { .. }),
        "{failure}"
    );
}

fn client() -> OAuthClient<'static> {
    OAuthClient {
        client_id: "client",
        client_secret: "secret",
        scopes: &[],
        redirect_uri: "http://localhost/cb",
    }
}
