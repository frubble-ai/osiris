//! The authenticated user, read from `GET /user`, and anyone's public profile, read from
//! `GET /users/{username}`, put in our terms.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use super::{
    GetCurrentUser, GetUser, mocks::generate_github_public_user::generate_github_public_user,
};
use crate::github::{
    mocks::github_on::github_on,
    mocks::{MockGithubNetwork, generate_github_user::generate_github_current_user},
};
use wiremock::MockServer;

#[tokio::test]
async fn reads_the_authenticated_user() {
    let server = MockServer::start().await;
    let answer = generate_github_current_user(7);
    MockGithubNetwork::on(&server)
        .answer("GET", "/user", &[], 200, answer.clone())
        .await;

    let user = github_on(&server)
        .get_current_user(&reqwest::Client::new(), "token", &GetCurrentUser {})
        .await
        .unwrap();

    assert_eq!(user.id, "7");
    assert_eq!(user.login, answer["login"].as_str().unwrap());
    assert_eq!(user.name.as_deref(), answer["name"].as_str());
    assert_eq!(user.account_type, "User");
}

#[tokio::test]
async fn reads_a_users_public_profile_by_login() {
    let server = MockServer::start().await;
    let answer = generate_github_public_user(8);
    MockGithubNetwork::on(&server)
        .answer("GET", "/users/mona%5Bbot%5D", &[], 200, answer.clone())
        .await;
    let input = GetUser {
        username: "mona[bot]".into(),
    };

    let user = github_on(&server)
        .get_user(&reqwest::Client::new(), "token", &input)
        .await
        .unwrap();

    assert_eq!(user.id, "8");
    assert_eq!(user.url, answer["html_url"].as_str().unwrap());
    assert_eq!(user.followers, answer["followers"].as_i64().unwrap());
    assert_eq!(user.public_repos, answer["public_repos"].as_i64().unwrap());
    assert!(GetUser::SCOPES.is_empty());
}
