//! Against the real providers, with test accounts' tokens: what the fakes can't tell us (an API
//! that changed its shape, a scope that stopped being enough). `#[ignore]`: run nightly, or with
//! `cargo nextest run -p osiris_providers --run-ignored only`. Locally, a provider whose token is
//! unset is skipped with a note; in CI (`CI` set) a missing token fails the test, so the nightly run
//! can't pass having checked nothing.

#![allow(
    clippy::unwrap_used,
    clippy::print_stderr,
    reason = "test fixtures: a failure here is the test failing; a skip is reported on stderr"
)]

use osiris_providers::{
    ListLimits, ProviderUrls,
    github::{
        Github,
        installations::{ListInstallations, ListRepositories},
    },
    linear::Linear,
    slack::Slack,
};
use url::Url;

fn token(name: &str) -> Option<String> {
    let token = std::env::var(name).ok().filter(|value| !value.is_empty());
    if token.is_none() {
        assert!(std::env::var_os("CI").is_none(), "{name} is not set in CI");
        eprintln!("skipped: {name} is not set");
    }
    token
}

const LIMITS: ListLimits = ListLimits {
    concurrency: 4,
    max_items: 5000,
    max_log_bytes: 262_144,
};

fn urls(oauth: &str, api: &str) -> ProviderUrls {
    ProviderUrls {
        oauth_base: Url::parse(oauth).unwrap(),
        api_base: Url::parse(api).unwrap(),
    }
}

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
        .list_repositories(&http, &token, &ListRepositories {})
        .await
        .unwrap();
}

#[tokio::test]
#[ignore = "live: needs OSIRIS_LIVE_LINEAR_TOKEN"]
async fn linear_lists_teams() {
    let Some(token) = token("OSIRIS_LIVE_LINEAR_TOKEN") else {
        return;
    };
    let linear = Linear::new(
        &urls("https://linear.app", "https://api.linear.app"),
        LIMITS,
    )
    .unwrap();
    assert!(
        !linear
            .list_teams(&reqwest::Client::new(), &token)
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
#[ignore = "live: needs OSIRIS_LIVE_SLACK_TOKEN"]
async fn slack_lists_channels() {
    let Some(token) = token("OSIRIS_LIVE_SLACK_TOKEN") else {
        return;
    };
    let slack = Slack::new(&urls("https://slack.com", "https://slack.com/api/"), LIMITS).unwrap();
    assert!(
        !slack
            .list_channels(&reqwest::Client::new(), &token)
            .await
            .unwrap()
            .is_empty()
    );
}
