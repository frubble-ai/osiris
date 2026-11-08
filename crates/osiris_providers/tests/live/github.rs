//! GitHub, live: the app's installations and their repositories.

use super::{limits::LIMITS, token::token, urls::urls};
use osiris_providers::{
    ActingAs,
    github::{
        Github,
        installations::{ListInstallations, ListRepositories},
    },
};

#[tokio::test]
#[ignore = "live: needs OSIRIS_LIVE_GITHUB_TOKEN"]
async fn github_lists_installations_and_their_repositories() {
    let Some(token) = token("OSIRIS_LIVE_GITHUB_TOKEN") else {
        return;
    };
    let github = Github::new(
        &urls("https://github.com", "https://api.github.com"),
        LIMITS,
    )
    .unwrap();
    let http = reqwest::Client::builder()
        .user_agent("osiris-live-tests")
        .build()
        .unwrap();
    let installations = github
        .list_installations(&http, &token, &ListInstallations {})
        .await
        .unwrap();
    assert!(
        !installations.is_empty(),
        "the test account has the app installed somewhere"
    );
    github
        .list_repositories(&http, &token, &ListRepositories {}, ActingAs::User)
        .await
        .unwrap();
}
