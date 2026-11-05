//! Linear against a fake Linear.

#![allow(
    clippy::unwrap_used,
    reason = "test fixtures: a failure here is the test failing"
)]

use osiris_providers::{ListLimits, ProviderError, ProviderUrls, linear::Linear};
use serde_json::json;
use url::Url;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_partial_json, method, path},
};

fn linear(server: &MockServer) -> Linear {
    let base = Url::parse(&server.uri()).unwrap();
    Linear::new(
        &ProviderUrls {
            oauth_base: base.clone(),
            api_base: base,
        },
        LIMITS,
    )
    .unwrap()
}

const LIMITS: ListLimits = ListLimits {
    concurrency: 2,
    max_items: 5000,
};

#[tokio::test]
async fn lists_teams_across_pages() {
    let server = MockServer::start().await;
    Mock::given(method("POST")).and(path("/graphql")).and(body_partial_json(json!({ "variables": { "after": null } })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "data": { "teams": { "nodes": [{ "id": "t1", "name": "Core" }], "pageInfo": { "hasNextPage": true, "endCursor": "c1" } } } })))
        .mount(&server).await;
    Mock::given(method("POST")).and(path("/graphql")).and(body_partial_json(json!({ "variables": { "after": "c1" } })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "data": { "teams": { "nodes": [{ "id": "t2", "name": "Ops" }], "pageInfo": { "hasNextPage": false, "endCursor": null } } } })))
        .mount(&server).await;

    let teams = linear(&server)
        .list_teams(&reqwest::Client::new(), "token")
        .await
        .unwrap();
    assert_eq!(
        teams
            .iter()
            .map(|team| team.name.as_str())
            .collect::<Vec<_>>(),
        ["Core", "Ops"]
    );
}

#[tokio::test]
async fn reads_an_authentication_error_beside_a_200_as_a_rejected_token() {
    let server = MockServer::start().await;
    Mock::given(path("/graphql"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "errors": [{ "message": "Authentication required", "extensions": { "code": "AUTHENTICATION_ERROR" } }] })))
        .mount(&server).await;
    let error = linear(&server)
        .list_teams(&reqwest::Client::new(), "token")
        .await
        .unwrap_err();
    assert!(
        matches!(error, ProviderError::Unauthorized { .. }),
        "{error}"
    );
}
