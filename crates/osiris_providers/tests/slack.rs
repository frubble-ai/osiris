//! Slack against a fake Slack.

#![allow(
    clippy::unwrap_used,
    reason = "test fixtures: a failure here is the test failing"
)]

use osiris_providers::{
    ListLimits, OAuthClient, Provider, ProviderError, ProviderUrls, slack::Slack,
};
use serde_json::json;
use url::Url;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, query_param},
};

fn slack(server: &MockServer) -> Slack {
    let base = Url::parse(&server.uri()).unwrap();
    Slack::new(
        &ProviderUrls {
            oauth_base: base.clone(),
            api_base: base.join("/api/").unwrap(),
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
async fn lists_channels_across_cursor_pages() {
    let server = MockServer::start().await;
    Mock::given(method("GET")).and(path("/api/conversations.list")).and(query_param("cursor", ""))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "ok": true, "channels": [{ "id": "C1", "name": "general", "is_private": false, "num_members": 3 }], "response_metadata": { "next_cursor": "n1" } })))
        .mount(&server).await;
    Mock::given(method("GET")).and(path("/api/conversations.list")).and(query_param("cursor", "n1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "ok": true, "channels": [{ "id": "C2", "is_private": true }], "response_metadata": { "next_cursor": "" } })))
        .mount(&server).await;

    let channels = slack(&server)
        .list_channels(&reqwest::Client::new(), "token")
        .await
        .unwrap();
    assert_eq!(channels.len(), 2);
    assert_eq!(
        channels[1].name, "C2",
        "a channel without a name shows its id"
    );
}

#[tokio::test]
async fn reads_ok_false_by_its_error() {
    let server = MockServer::start().await;
    Mock::given(path("/api/conversations.list"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({ "ok": false, "error": "token_revoked" })),
        )
        .mount(&server)
        .await;
    assert!(matches!(
        slack(&server)
            .list_channels(&reqwest::Client::new(), "token")
            .await
            .unwrap_err(),
        ProviderError::Unauthorized { .. }
    ));

    server.reset().await;
    Mock::given(path("/api/conversations.list"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({ "ok": false, "error": "missing_scope" })),
        )
        .mount(&server)
        .await;
    assert!(matches!(
        slack(&server)
            .list_channels(&reqwest::Client::new(), "token")
            .await
            .unwrap_err(),
        ProviderError::Refused { .. }
    ));
}

#[tokio::test]
async fn counts_a_token_already_gone_as_revoked() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/auth.revoke"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({ "ok": false, "error": "invalid_auth" })),
        )
        .mount(&server)
        .await;
    let client = OAuthClient {
        client_id: "c",
        client_secret: "s",
        scopes: &[],
        redirect_uri: "http://localhost/cb",
    };
    slack(&server)
        .revoke(&reqwest::Client::new(), &client, "token")
        .await
        .unwrap();
}

#[tokio::test]
async fn stops_reading_channels_at_the_cap() {
    let server = MockServer::start().await;
    let channels: Vec<_> = (0..3)
        .map(|n| json!({ "id": format!("C{n}"), "name": format!("c{n}"), "is_private": false }))
        .collect();
    // A first page only: a second request would fail the list.
    Mock::given(method("GET")).and(path("/api/conversations.list"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "ok": true, "channels": channels, "response_metadata": { "next_cursor": "n1" } })))
        .expect(1)
        .mount(&server).await;
    let base = Url::parse(&server.uri()).unwrap();
    let capped = Slack::new(
        &ProviderUrls {
            oauth_base: base.clone(),
            api_base: base.join("/api/").unwrap(),
        },
        ListLimits {
            concurrency: 1,
            max_items: 2,
        },
    )
    .unwrap();
    let channels = capped
        .list_channels(&reqwest::Client::new(), "token")
        .await
        .unwrap();
    assert_eq!(channels.len(), 2);
}
