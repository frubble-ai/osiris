//! `slack.list_channels` on a Slack connection.

use crate::testing::TestContext;
use crate::testing::golden::{GoldenSession, Key};
use osiris_providers::slack::mocks::MockSlackNetwork;
use serde_json::json;

pub async fn run(app: &TestContext, session: &mut GoldenSession<'_>) {
    let (id, _) = app.connected_to_slack().await;
    // The connection's own `connection.active` is arranged, not part of the transcript.
    session.skip_events();
    MockSlackNetwork::on(&app.provider)
        .answer_channels_page(
            None,
            &[json!({
                "id": "CGOLDEN",
                "name": "general",
                "is_private": false,
                "num_members": 3,
            })],
            "",
            None,
        )
        .await;
    session
        .send(
            "POST",
            &format!("/v1/connected-accounts/{id}/actions/slack.list_channels"),
            Key::Api,
            None,
            &[],
        )
        .await;
}

golden!("slack/list_channels", run);
