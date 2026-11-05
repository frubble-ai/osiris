//! Against the real providers, with test accounts' tokens: what the fakes can't tell us (an API
//! that changed its shape, a scope that stopped being enough). `#[ignore]`: run nightly, or with
//! `cargo nextest run -p osiris_providers --run-ignored only`. A provider whose token is unset is
//! skipped.

#![allow(
    clippy::unwrap_used,
    reason = "test fixtures: a failure here is the test failing"
)]

use osiris_providers::{ListLimits, ProviderUrls, github::Github, linear::Linear, slack::Slack};
use url::Url;

fn token(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|value| !value.is_empty())
}

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
        ListLimits {
            concurrency: 4,
            max_items: 5000,
        },
    )
    .unwrap();
    let http = reqwest::Client::builder()
        .user_agent("osiris-live-tests")
        .build()
        .unwrap();
    let installations = github.list_installations(&http, &token).await.unwrap();
    assert!(
        !installations.is_empty(),
        "the test account has the app installed somewhere"
    );
    github.list_repositories(&http, &token).await.unwrap();
}

#[tokio::test]
#[ignore = "live: needs OSIRIS_LIVE_LINEAR_TOKEN"]
async fn linear_lists_teams() {
    let Some(token) = token("OSIRIS_LIVE_LINEAR_TOKEN") else {
        return;
    };
    let linear = Linear::new(&urls("https://linear.app", "https://api.linear.app")).unwrap();
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
    let slack = Slack::new(&urls("https://slack.com", "https://slack.com/api/")).unwrap();
    assert!(
        !slack
            .list_channels(&reqwest::Client::new(), &token)
            .await
            .unwrap()
            .is_empty()
    );
}
