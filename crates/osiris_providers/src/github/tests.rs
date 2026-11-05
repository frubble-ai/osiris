//! GitHub against its faked network.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use super::{
    Github,
    mocks::{
        MockGithubNetwork, generate_github_installation::generate_github_installation,
        generate_github_repository::generate_github_repository,
    },
    types::GithubAccountType,
};
use crate::{ListLimits, OAuthClient, Provider, ProviderError, ProviderUrls};
use rstest::rstest;
use url::Url;
use wiremock::MockServer;

const LIMITS: ListLimits = ListLimits {
    concurrency: 2,
    max_items: 5000,
};

fn github(server: &MockServer, limits: ListLimits) -> Github {
    let base = Url::parse(&server.uri()).unwrap();
    Github::new(
        &ProviderUrls {
            oauth_base: base.clone(),
            api_base: base,
        },
        limits,
    )
    .unwrap()
}

#[tokio::test]
async fn lists_the_installations_the_user_can_see_with_their_token() {
    let server = MockServer::start().await;
    let installations = [
        generate_github_installation(1, false),
        generate_github_installation(2, true),
    ];
    MockGithubNetwork::on(&server)
        .answer_installations_for_token("the-token", &installations)
        .await;

    let listed = github(&server, LIMITS)
        .list_installations(&reqwest::Client::new(), "the-token")
        .await
        .unwrap();

    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].account_type, GithubAccountType::User);
    assert_eq!(
        listed[1].account,
        installations[1]["account"]["login"].as_str().unwrap()
    );
}

#[tokio::test]
async fn lists_every_repository_once_across_installations_and_pages() {
    let server = MockServer::start().await;
    let network = MockGithubNetwork::on(&server);
    network
        .answer_installations(&[
            generate_github_installation(1, false),
            generate_github_installation(2, true),
        ])
        .await;
    let first: Vec<_> = (1..=100).map(generate_github_repository).collect();
    network
        .answer_repositories(1, Some(1), 102, &first, None)
        .await;
    network
        .answer_repositories(
            1,
            Some(2),
            102,
            &[
                generate_github_repository(101),
                generate_github_repository(102),
            ],
            None,
        )
        .await;
    // The second installation shares a repository with the first.
    network
        .answer_repositories(2, None, 1, &[generate_github_repository(102)], None)
        .await;

    let repositories = github(&server, LIMITS)
        .list_repositories(&reqwest::Client::new(), "token")
        .await
        .unwrap();

    assert_eq!(repositories.len(), 102);
    assert_eq!(repositories.iter().filter(|r| r.id == "102").count(), 1);
}

#[tokio::test]
async fn stops_reading_repositories_at_the_cap() {
    let server = MockServer::start().await;
    let network = MockGithubNetwork::on(&server);
    network
        .answer_installations(&[generate_github_installation(1, false)])
        .await;
    let first: Vec<_> = (1..=100).map(generate_github_repository).collect();
    // Only the first page is read: a second request would fail the test.
    network
        .answer_repositories(1, Some(1), 300, &first, Some(1))
        .await;

    let repositories = github(
        &server,
        ListLimits {
            concurrency: 1,
            max_items: 50,
        },
    )
    .list_repositories(&reqwest::Client::new(), "token")
    .await
    .unwrap();

    assert_eq!(repositories.len(), 50);
}

#[rstest]
#[case::rejected_token(401, true)]
#[case::refused(403, false)]
#[tokio::test]
async fn tells_a_rejected_token_from_a_refusal(#[case] status: u16, #[case] rejected: bool) {
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .fail_installations(status, "Bad credentials")
        .await;

    let error = github(&server, LIMITS)
        .list_installations(&reqwest::Client::new(), "token")
        .await
        .unwrap_err();

    assert_eq!(
        matches!(error, ProviderError::Unauthorized { .. }),
        rejected,
        "{error}"
    );
}

#[rstest]
#[case::revoked_now(204)]
#[case::already_gone(404)]
#[tokio::test]
async fn ends_a_grant_or_counts_one_github_no_longer_has_as_ended(#[case] status: u16) {
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer_revoke(status, Some(1))
        .await;
    let client = OAuthClient {
        client_id: "client",
        client_secret: "secret",
        scopes: &[],
        redirect_uri: "http://localhost/cb",
    };

    let ended = github(&server, LIMITS)
        .revoke(&reqwest::Client::new(), &client, "token")
        .await;

    assert!(ended.is_ok());
}
