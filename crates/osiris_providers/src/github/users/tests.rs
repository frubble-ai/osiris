//! The authenticated user, read from `GET /user` and put in our terms.

#![allow(
    clippy::unwrap_used,
    reason = "tests: a failure here is the test failing"
)]

use super::GetCurrentUser;
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
