//! `github.list_installations` on a GitHub connection, and refused on another provider's
//! connection.

use crate::testing::TestContext;
use crate::testing::golden::{GoldenSession, Key};
use osiris_providers::github::mocks::MockGithubNetwork;
use serde_json::json;

pub async fn run(app: &TestContext, session: &mut GoldenSession<'_>) {
    let github = app.connected_to_github(28800).await;
    let other = app.connected(28800).await;
    // The connections' own `connection.active` events are arranged, not part of the transcript.
    session.skip_events();
    MockGithubNetwork::on(&app.provider)
        .answer_installations(&[json!({
            "id": 7,
            "account": { "login": "golden-org", "type": "Organization" },
            "repository_selection": "selected",
        })])
        .await;
    for id in [github, other] {
        session
            .send(
                "POST",
                &format!("/v1/connected-accounts/{id}/actions/github.list_installations"),
                Key::Api,
                None,
                &[],
            )
            .await;
    }
}

golden!("github/list_installations", run);
