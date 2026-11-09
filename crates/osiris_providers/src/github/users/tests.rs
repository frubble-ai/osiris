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
    mocks::generate_github::generate_github,
    mocks::{
        MockGithubNetwork, generate_github_names::generate_github_token,
        generate_github_user::generate_github_current_user,
    },
};
use serde_json::json;
use wiremock::MockServer;

#[tokio::test]
async fn reads_the_authenticated_user() {
    let server = MockServer::start().await;
    let mut answer = generate_github_public_user(7);
    answer["total_private_repos"] = json!(3);
    answer["owned_private_repos"] = json!(2);
    answer["private_gists"] = json!(1);
    answer["disk_usage"] = json!(1024);
    answer["collaborators"] = json!(0);
    answer["two_factor_authentication"] = json!(true);
    answer["plan"] = json!({ "name": "pro", "space": 976_562_499, "private_repos": 9999 });
    MockGithubNetwork::on(&server)
        .answer("GET", "/user", &[], 200, answer.clone())
        .await;

    let user = generate_github(&server)
        .get_current_user(
            &reqwest::Client::new(),
            &generate_github_token(),
            &GetCurrentUser {},
        )
        .await
        .unwrap();

    assert_eq!(user.profile.id, "7");
    assert_eq!(user.profile.login, answer["login"].as_str().unwrap());
    assert_eq!(user.profile.name.as_deref(), answer["name"].as_str());
    assert_eq!(user.profile.account_type, "User");
    assert_eq!(
        user.profile.followers,
        answer["followers"].as_i64().unwrap()
    );
    assert_eq!(user.profile.blog.as_deref(), Some("https://example.com"));
    assert_eq!(
        (
            user.total_private_repos,
            user.owned_private_repos,
            user.private_gists
        ),
        (Some(3), Some(2), Some(1))
    );
    assert_eq!((user.disk_usage, user.collaborators), (Some(1024), Some(0)));
    assert_eq!(user.two_factor_authentication, Some(true));
    assert_eq!(user.plan.as_deref(), Some("pro"));
    let answered = serde_json::to_value(&user).unwrap();
    assert_eq!(answered["login"], answer["login"]);
    assert_eq!(answered["plan"], "pro");
}

#[tokio::test]
async fn reads_the_authenticated_user_where_the_token_may_not_see_the_private_parts() {
    let server = MockServer::start().await;
    MockGithubNetwork::on(&server)
        .answer("GET", "/user", &[], 200, generate_github_current_user(9))
        .await;

    let user = generate_github(&server)
        .get_current_user(
            &reqwest::Client::new(),
            &generate_github_token(),
            &GetCurrentUser {},
        )
        .await
        .unwrap();

    assert_eq!(user.profile.id, "9");
    assert_eq!(user.total_private_repos, None);
    assert_eq!(user.plan, None);
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

    let user = generate_github(&server)
        .get_user(&reqwest::Client::new(), &generate_github_token(), &input)
        .await
        .unwrap();

    assert_eq!(user.id, "8");
    assert_eq!(user.url, answer["html_url"].as_str().unwrap());
    assert_eq!(user.followers, answer["followers"].as_i64().unwrap());
    assert_eq!(user.public_repos, answer["public_repos"].as_i64().unwrap());
    assert_eq!(user.public_gists, answer["public_gists"].as_i64().unwrap());
    assert_eq!(user.blog.as_deref(), Some("https://example.com"));
    assert_eq!(user.twitter_username.as_deref(), Some("mona"));
    assert_eq!(user.hireable, Some(true));
    assert!(!user.site_admin);
    assert_eq!(user.created_at.as_deref(), Some("2019-01-01T00:00:00Z"));
    assert_eq!(user.updated_at.as_deref(), Some("2026-01-01T00:00:00Z"));
    assert!(GetUser::SCOPES.is_empty());
}
