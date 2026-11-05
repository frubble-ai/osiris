//! GitHub against a fake GitHub.

#![allow(
    clippy::unwrap_used,
    reason = "test fixtures: a failure here is the test failing"
)]

use osiris_providers::{
    ListLimits, OAuthClient, Provider, ProviderError, ProviderUrls,
    github::{Github, types::GithubAccountType},
};
use serde_json::json;
use url::Url;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path, query_param},
};

const LIMITS: ListLimits = ListLimits {
    concurrency: 2,
    max_items: 5000,
};

fn github(server: &MockServer) -> Github {
    github_with(server, LIMITS)
}

fn github_with(server: &MockServer, limits: ListLimits) -> Github {
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

fn repository(id: u64) -> serde_json::Value {
    json!({ "id": id, "full_name": format!("acme/r{id}"), "private": id.is_multiple_of(2), "html_url": format!("https://github.com/acme/r{id}"), "description": null })
}

#[tokio::test]
async fn lists_installations_with_the_token() {
    let server = MockServer::start().await;
    Mock::given(method("GET")).and(path("/user/installations")).and(header("authorization", "Bearer token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total_count": 2,
            "installations": [
                { "id": 1, "account": { "login": "dan", "type": "User" }, "repository_selection": "all" },
                { "id": 2, "account": { "login": "acme", "type": "Organization" }, "repository_selection": "selected" }
            ]
        })))
        .mount(&server).await;

    let installations = github(&server)
        .list_installations(&reqwest::Client::new(), "token")
        .await
        .unwrap();
    assert_eq!(installations.len(), 2);
    assert_eq!(installations[0].account_type, GithubAccountType::User);
    assert_eq!(installations[1].account, "acme");
}

#[tokio::test]
async fn lists_every_repository_once_across_installations_and_pages() {
    let server = MockServer::start().await;
    Mock::given(method("GET")).and(path("/user/installations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "total_count": 2, "installations": [
            { "id": 1, "account": { "login": "a", "type": "User" }, "repository_selection": "all" },
            { "id": 2, "account": { "login": "b", "type": "Organization" }, "repository_selection": "all" } ] })))
        .mount(&server).await;
    let first: Vec<_> = (1..=100).map(repository).collect();
    Mock::given(method("GET"))
        .and(path("/user/installations/1/repositories"))
        .and(query_param("page", "1"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({ "total_count": 102, "repositories": first })),
        )
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/user/installations/1/repositories"))
        .and(query_param("page", "2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            json!({ "total_count": 102, "repositories": [repository(101), repository(102)] }),
        ))
        .mount(&server)
        .await;
    // The second installation shares a repository with the first.
    Mock::given(method("GET"))
        .and(path("/user/installations/2/repositories"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({ "total_count": 1, "repositories": [repository(102)] })),
        )
        .mount(&server)
        .await;

    let repositories = github(&server)
        .list_repositories(&reqwest::Client::new(), "token")
        .await
        .unwrap();
    assert_eq!(repositories.len(), 102);
    assert_eq!(repositories.iter().filter(|r| r.id == "102").count(), 1);
}

#[tokio::test]
async fn tells_a_rejected_token_from_a_refusal() {
    let server = MockServer::start().await;
    Mock::given(path("/user/installations"))
        .respond_with(ResponseTemplate::new(401).set_body_string("Bad credentials"))
        .mount(&server)
        .await;
    let error = github(&server)
        .list_installations(&reqwest::Client::new(), "token")
        .await
        .unwrap_err();
    assert!(
        matches!(error, ProviderError::Unauthorized { .. }),
        "{error}"
    );
}

#[tokio::test]
async fn counts_a_grant_github_no_longer_has_as_revoked() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/applications/client/grant"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;
    let client = OAuthClient {
        client_id: "client",
        client_secret: "secret",
        scopes: &[],
        redirect_uri: "http://localhost/cb",
    };
    github(&server)
        .revoke(&reqwest::Client::new(), &client, "token")
        .await
        .unwrap();
}

#[tokio::test]
async fn stops_reading_repositories_at_the_cap() {
    let server = MockServer::start().await;
    Mock::given(method("GET")).and(path("/user/installations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "total_count": 1, "installations": [
            { "id": 1, "account": { "login": "a", "type": "User" }, "repository_selection": "all" } ] })))
        .mount(&server).await;
    let first: Vec<_> = (1..=100).map(repository).collect();
    // Only the first page is served: a second request would fail the list.
    Mock::given(method("GET"))
        .and(path("/user/installations/1/repositories"))
        .and(query_param("page", "1"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({ "total_count": 300, "repositories": first })),
        )
        .expect(1)
        .mount(&server)
        .await;

    let repositories = github_with(
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
