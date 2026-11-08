//! GitHub against its faked network.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use super::{
    Github,
    installations::{ListInstallations, ListRepositories},
    mocks::{
        MockGithubNetwork, generate_github_installation::generate_github_installation,
        generate_github_repository::generate_github_repository,
    },
};
use crate::{ActingAs, ListLimits, OAuthClient, Provider, ProviderError, ProviderUrls};
use rstest::rstest;
use url::Url;
use wiremock::MockServer;

const LIMITS: ListLimits = ListLimits {
    concurrency: 2,
    max_items: 5000,
    max_log_bytes: 262_144,
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
        .list_installations(&reqwest::Client::new(), "the-token", &ListInstallations {})
        .await
        .unwrap();

    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].account_type, "user");
    assert_eq!(listed[1].account_type, "organization");
    assert_eq!(listed[1].repository_selection, "selected");
    assert_eq!(
        listed[1].account,
        installations[1]["account"]["login"].as_str().unwrap()
    );
}

#[rstest]
#[case::enterprise("Enterprise", "everything_new", "organization")]
#[case::a_new_type("Bot", "all", "bot")]
#[tokio::test]
async fn passes_values_github_adds_on_instead_of_refusing_the_page(
    #[case] kind: &str,
    #[case] selection: &str,
    #[case] account_type: &str,
) {
    let server = MockServer::start().await;
    let mut installation = generate_github_installation(1, true);
    installation["account"]["type"] = kind.into();
    installation["repository_selection"] = selection.into();
    MockGithubNetwork::on(&server)
        .answer_installations_for_token("the-token", &[installation])
        .await;

    let listed = github(&server, LIMITS)
        .list_installations(&reqwest::Client::new(), "the-token", &ListInstallations {})
        .await
        .unwrap();

    assert_eq!(listed[0].account_type, account_type);
    assert_eq!(listed[0].repository_selection, selection);
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
        .list_repositories(
            &reqwest::Client::new(),
            "token",
            &ListRepositories {},
            ActingAs::User,
        )
        .await
        .unwrap();

    assert_eq!(repositories.len(), 102);
    assert_eq!(
        repositories
            .iter()
            .filter(|repository| repository.id == "102")
            .count(),
        1
    );
}

#[tokio::test]
async fn reads_an_installation_s_own_repositories_on_an_installation_connection() {
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/installation/repositories",
            &[("per_page", "100"), ("page", "1")],
            200,
            serde_json::json!({ "total_count": 1, "repositories": [generate_github_repository(8)] }),
        )
        .await;

    let repositories = github(&server, LIMITS)
        .list_repositories(
            &reqwest::Client::new(),
            "token",
            &ListRepositories {},
            ActingAs::Installation,
        )
        .await
        .unwrap();

    assert_eq!(repositories.len(), 1);
    assert_eq!(repositories[0].id, "8");
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
            max_log_bytes: 262_144,
        },
    )
    .list_repositories(
        &reqwest::Client::new(),
        "token",
        &ListRepositories {},
        ActingAs::User,
    )
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
        .list_installations(&reqwest::Client::new(), "token", &ListInstallations {})
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
