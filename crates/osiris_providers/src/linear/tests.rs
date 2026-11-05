//! Linear against its faked network.

#![allow(
    clippy::unwrap_used,
    reason = "tests: a failure here is the test failing"
)]

use super::{
    Linear,
    mocks::{MockLinearNetwork, generate_linear_team::generate_linear_team},
};
use crate::{ListLimits, ProviderError, ProviderUrls};
use serde_json::json;
use url::Url;
use wiremock::MockServer;

fn linear(server: &MockServer) -> Linear {
    let base = Url::parse(&server.uri()).unwrap();
    Linear::new(
        &ProviderUrls {
            oauth_base: base.clone(),
            api_base: base,
        },
        ListLimits {
            concurrency: 2,
            max_items: 5000,
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
        .answer_teams_page(None, std::slice::from_ref(&first), Some("c1"))
        .await;
    network
        .answer_teams_page(Some("c1"), std::slice::from_ref(&second), None)
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
