//! Linear, live: the workspace's teams.

use super::{limits::LIMITS, token::token, urls::urls};
use osiris_providers::linear::{Linear, teams::ListTeams};

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
            .list_teams(&reqwest::Client::new(), &token, &ListTeams {})
            .await
            .unwrap()
            .is_empty()
    );
}
