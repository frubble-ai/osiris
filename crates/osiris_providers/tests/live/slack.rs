//! Slack, live: the workspace's channels.

use super::{limits::LIMITS, token::token, urls::urls};
use osiris_providers::slack::{Slack, conversations::ListChannels};

#[tokio::test]
#[ignore = "live: needs OSIRIS_LIVE_SLACK_TOKEN"]
async fn slack_lists_channels() {
    let Some(token) = token("OSIRIS_LIVE_SLACK_TOKEN") else {
        return;
    };
    let slack = Slack::new(&urls("https://slack.com", "https://slack.com/api/"), LIMITS).unwrap();
    assert!(
        !slack
            .list_channels(&reqwest::Client::new(), &token, &ListChannels {})
            .await
            .unwrap()
            .is_empty()
    );
}
