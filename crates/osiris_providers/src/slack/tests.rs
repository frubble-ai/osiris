//! Slack against its faked network.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use super::{
    Slack,
    mocks::{MockSlackNetwork, generate_slack_channel::generate_slack_channel},
};
use crate::{ListLimits, OAuthClient, Provider, ProviderError, ProviderUrls};
use rstest::rstest;
use serde_json::json;
use url::Url;
use wiremock::MockServer;

fn slack(server: &MockServer, max_items: usize) -> Slack {
    let base = Url::parse(&server.uri()).unwrap();
    Slack::new(
        &ProviderUrls {
            oauth_base: base.clone(),
            api_base: base.join("/api/").unwrap(),
        },
        ListLimits {
            concurrency: 1,
            max_items,
        },
    )
    .unwrap()
}

#[tokio::test]
async fn lists_channels_across_cursor_pages_naming_an_unnamed_one_by_its_id() {
    let server = MockServer::start().await;
    let network = MockSlackNetwork::on(&server);
    let named = generate_slack_channel();
    let mut unnamed = generate_slack_channel();
    unnamed.as_object_mut().unwrap().remove("name");
    network
        .answer_channels_page(Some(""), &[named], "n1", None)
        .await;
    network
        .answer_channels_page(Some("n1"), &[unnamed.clone()], "", None)
        .await;

    let channels = slack(&server, 5000)
        .list_channels(&reqwest::Client::new(), "token")
        .await
        .unwrap();

    assert_eq!(channels.len(), 2);
    assert_eq!(channels[1].name, unnamed["id"].as_str().unwrap());
}

#[rstest]
#[case::revoked_token("token_revoked", true)]
#[case::missing_scope("missing_scope", false)]
#[tokio::test]
async fn reads_ok_false_by_its_error(#[case] error: &str, #[case] rejected: bool) {
    let server = MockServer::start().await;
    MockSlackNetwork::on(&server)
        .answer_channels_error(error)
        .await;

    let failure = slack(&server, 5000)
        .list_channels(&reqwest::Client::new(), "token")
        .await
        .unwrap_err();

    if rejected {
        assert!(
            matches!(failure, ProviderError::Unauthorized { .. }),
            "{failure}"
        );
    } else {
        assert!(
            matches!(failure, ProviderError::Refused { .. }),
            "{failure}"
        );
    }
}

#[tokio::test]
async fn counts_a_token_already_gone_as_revoked() {
    let server = MockServer::start().await;
    MockSlackNetwork::on(&server)
        .answer_revoke(json!({ "ok": false, "error": "invalid_auth" }))
        .await;
    let client = OAuthClient {
        client_id: "c",
        client_secret: "s",
        scopes: &[],
        redirect_uri: "http://localhost/cb",
    };

    let ended = slack(&server, 5000)
        .revoke(&reqwest::Client::new(), &client, "token")
        .await;

    assert!(ended.is_ok());
}

#[tokio::test]
async fn stops_reading_channels_at_the_cap() {
    let server = MockServer::start().await;
    let channels: Vec<_> = (0..3).map(|_| generate_slack_channel()).collect();
    // Only the first page is read: a second request would fail the test.
    MockSlackNetwork::on(&server)
        .answer_channels_page(None, &channels, "n1", Some(1))
        .await;

    let listed = slack(&server, 2)
        .list_channels(&reqwest::Client::new(), "token")
        .await
        .unwrap();

    assert_eq!(listed.len(), 2);
}
