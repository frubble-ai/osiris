//! Searches on GitHub, each asked with its query, sort and page, and answered in our terms with
//! the count GitHub gave.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use super::{
    SearchCode, SearchCommits, SearchLabels, SearchRepositories, SearchTopics, SearchUsers,
    mocks::{
        generate_github_search_code::generate_github_search_code,
        generate_github_search_commit::generate_github_search_commit,
        generate_github_search_label::generate_github_search_label,
        generate_github_search_page::generate_github_search_page,
        generate_github_search_repository::generate_github_search_repository,
        generate_github_search_topic::generate_github_search_topic,
        generate_github_search_user::generate_github_search_user,
    },
};
use crate::github::github_id::GithubId;
use crate::github::mocks::{
    MockGithubNetwork,
    generate_github::generate_github,
    generate_github_names::{generate_github_login, generate_github_repo, generate_github_token},
};
use rstest::rstest;
use serde_json::json;
use wiremock::MockServer;

#[rstest]
#[case::complete(false)]
#[case::incomplete(true)]
#[tokio::test]
async fn searches_repositories_sorted_as_asked(#[case] incomplete: bool) {
    let token = generate_github_token();
    let server = MockServer::start().await;
    let items = [
        generate_github_search_repository(1),
        generate_github_search_repository(2),
    ];
    let page = generate_github_search_page(&items, incomplete);
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/search/repositories",
            &[
                ("q", "osiris language:rust"),
                ("sort", "stars"),
                ("order", "desc"),
                ("per_page", "2"),
                ("page", "1"),
            ],
            200,
            page.clone(),
        )
        .await;
    let input = SearchRepositories {
        query: "osiris language:rust".into(),
        sort: Some("stars".into()),
        order: Some("desc".into()),
        per_page: Some(2),
        page: Some(1),
    };

    let found = generate_github(&server)
        .search_repositories(&reqwest::Client::new(), &token, &input)
        .await
        .unwrap();

    assert_eq!(found.total_count, page["total_count"].as_i64().unwrap());
    assert_eq!(found.incomplete_results, incomplete);
    assert_eq!(found.repositories.len(), 2);
    assert_eq!(found.repositories[0].id, "1");
    assert_eq!(
        found.repositories[0].full_name,
        items[0]["full_name"].as_str().unwrap()
    );
    assert_eq!(
        found.repositories[0].stargazers_count,
        items[0]["stargazers_count"].as_i64().unwrap()
    );
    assert_eq!(found.repositories[0].language.as_deref(), Some("Rust"));
}

#[tokio::test]
async fn searches_users() {
    let token = generate_github_token();
    let server = MockServer::start().await;
    let items = [generate_github_search_user(3)];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/search/users",
            &[("q", "location:berlin"), ("sort", "followers")],
            200,
            generate_github_search_page(&items, false),
        )
        .await;
    let input = SearchUsers {
        query: "location:berlin".into(),
        sort: Some("followers".into()),
        order: None,
        per_page: None,
        page: None,
    };

    let found = generate_github(&server)
        .search_users(&reqwest::Client::new(), &token, &input)
        .await
        .unwrap();

    assert_eq!(found.items[0].id, "3");
    assert_eq!(found.items[0].login, items[0]["login"].as_str().unwrap());
    assert_eq!(found.items[0].account_type, "User");
}

#[tokio::test]
async fn searches_commits() {
    let token = generate_github_token();
    let query = format!(
        "repo:{}/{} fix",
        generate_github_login(),
        generate_github_repo()
    );
    let server = MockServer::start().await;
    let items = [generate_github_search_commit(4)];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/search/commits",
            &[("q", query.as_str()), ("order", "asc")],
            200,
            generate_github_search_page(&items, false),
        )
        .await;
    let input = SearchCommits {
        query: query.clone(),
        sort: None,
        order: Some("asc".into()),
        per_page: None,
        page: None,
    };

    let found = generate_github(&server)
        .search_commits(&reqwest::Client::new(), &token, &input)
        .await
        .unwrap();

    let commit = &found.items[0];
    assert_eq!(commit.sha, items[0]["sha"].as_str().unwrap());
    assert_eq!(
        commit.message,
        items[0]["commit"]["message"].as_str().unwrap()
    );
    assert_eq!(commit.url, items[0]["html_url"].as_str().unwrap());
    assert_eq!(commit.repository.id, "4");
}

#[tokio::test]
async fn searches_code() {
    let token = generate_github_token();
    let server = MockServer::start().await;
    let items = [generate_github_search_code(5)];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/search/code",
            &[("q", "fn main language:rust"), ("per_page", "1")],
            200,
            generate_github_search_page(&items, false),
        )
        .await;
    let input = SearchCode {
        query: "fn main language:rust".into(),
        sort: None,
        order: None,
        per_page: Some(1),
        page: None,
    };

    let found = generate_github(&server)
        .search_code(&reqwest::Client::new(), &token, &input)
        .await
        .unwrap();

    let file = &found.items[0];
    assert_eq!(file.path, items[0]["path"].as_str().unwrap());
    assert_eq!(file.url, items[0]["html_url"].as_str().unwrap());
    assert_eq!(file.repository.id, "5");
    assert_eq!(file.repository.owner.as_ref().unwrap().id, "6");
}

#[tokio::test]
async fn searches_the_labels_of_a_repository_by_its_id() {
    let token = generate_github_token();
    let server = MockServer::start().await;
    let items = [generate_github_search_label(7)];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/search/labels",
            &[
                ("repository_id", "64778136"),
                ("q", "bug"),
                ("sort", "created"),
            ],
            200,
            generate_github_search_page(&items, false),
        )
        .await;
    let input = SearchLabels {
        repository_id: GithubId(64_778_136),
        query: "bug".into(),
        sort: Some("created".into()),
        order: None,
        per_page: None,
        page: None,
    };

    let found = generate_github(&server)
        .search_labels(&reqwest::Client::new(), &token, &input)
        .await
        .unwrap();

    assert_eq!(found.items[0].id, "7");
    assert_eq!(found.items[0].color, items[0]["color"].as_str().unwrap());
    assert_eq!(found.items[0].description, None);
}

#[tokio::test]
async fn searches_topics() {
    let token = generate_github_token();
    let server = MockServer::start().await;
    let items = [generate_github_search_topic(true)];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/search/topics",
            &[("q", "is:featured"), ("page", "2")],
            200,
            generate_github_search_page(&items, false),
        )
        .await;
    let input = SearchTopics {
        query: "is:featured".into(),
        per_page: None,
        page: Some(2),
    };

    let found = generate_github(&server)
        .search_topics(&reqwest::Client::new(), &token, &input)
        .await
        .unwrap();

    assert_eq!(found.items[0].name, items[0]["name"].as_str().unwrap());
    assert!(found.items[0].featured);
}

#[tokio::test]
async fn a_search_answer_without_items_reads_as_none_found() {
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/search/topics",
            &[("q", "nothing")],
            200,
            json!({ "total_count": 0, "incomplete_results": false }),
        )
        .await;
    let input = SearchTopics {
        query: "nothing".into(),
        per_page: None,
        page: None,
    };

    let found = generate_github(&server)
        .search_topics(&reqwest::Client::new(), &token, &input)
        .await
        .unwrap();

    assert_eq!(found.total_count, 0);
    assert!(found.items.is_empty());
}

#[tokio::test]
async fn sends_the_sort_trimmed_and_leaves_out_a_blank_order() {
    let token = generate_github_token();
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer_filtered(
            "/search/repositories",
            &[("q", "osiris"), ("sort", "stars")],
            &["order"],
            generate_github_search_page(&[], false),
        )
        .await;
    let input = SearchRepositories {
        query: "osiris".into(),
        sort: Some("  stars ".into()),
        order: Some("  ".into()),
        per_page: None,
        page: None,
    };

    let found = generate_github(&server)
        .search_repositories(&reqwest::Client::new(), &token, &input)
        .await
        .unwrap();

    assert!(found.repositories.is_empty());
}
