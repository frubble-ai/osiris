//! A repository and its branches, read from GitHub and put in our terms, with every value placed
//! in the path percent-encoded.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use super::{GetBranch, GetRepository, ListBranches};
use crate::github::{
    mocks::github_on::github_on,
    mocks::{
        MockGithubNetwork, generate_github_branch::generate_github_branch,
        generate_github_repository::generate_github_repository,
    },
};
use wiremock::MockServer;

#[tokio::test]
async fn reads_a_repository_by_owner_and_name() {
    let server = MockServer::start().await;
    let answer = generate_github_repository(42);
    MockGithubNetwork::on(&server)
        .answer("GET", "/repos/octo/hello", &[], 200, answer.clone())
        .await;
    let input = GetRepository {
        owner: "octo".into(),
        repo: "hello".into(),
    };

    let repository = github_on(&server)
        .get_repository(&reqwest::Client::new(), "token", &input)
        .await
        .unwrap();

    assert_eq!(repository.id, "42");
    assert_eq!(repository.full_name, answer["full_name"].as_str().unwrap());
    assert_eq!(
        repository.owner.login,
        answer["owner"]["login"].as_str().unwrap()
    );
    assert_eq!(repository.default_branch, "main");
}

#[tokio::test]
async fn lists_a_page_of_branches_filtered_by_protection() {
    let server = MockServer::start().await;
    let branches = [generate_github_branch(true), generate_github_branch(true)];
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/branches",
            &[("protected", "true"), ("per_page", "2"), ("page", "3")],
            200,
            serde_json::json!(branches),
        )
        .await;
    let input = ListBranches {
        owner: "octo".into(),
        repo: "hello".into(),
        protected: Some(true),
        per_page: Some(2),
        page: Some(3),
    };

    let listed = github_on(&server)
        .list_branches(&reqwest::Client::new(), "token", &input)
        .await
        .unwrap();

    assert_eq!(listed.len(), 2);
    assert_eq!(listed[0].name, branches[0]["name"].as_str().unwrap());
    assert_eq!(
        listed[0].commit_sha,
        branches[0]["commit"]["sha"].as_str().unwrap()
    );
    assert!(listed[0].protected);
}

#[tokio::test]
async fn encodes_a_branch_name_with_a_slash_into_one_segment() {
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer(
            "GET",
            "/repos/octo/hello/branches/feature%2Flogin",
            &[],
            200,
            generate_github_branch(false),
        )
        .await;
    let input = GetBranch {
        owner: "octo".into(),
        repo: "hello".into(),
        branch: "feature/login".into(),
    };

    let branch = github_on(&server)
        .get_branch(&reqwest::Client::new(), "token", &input)
        .await
        .unwrap();

    assert!(!branch.protected);
}
