//! GitHub's actions on a live connection: its installations and their repositories, and an action
//! of another provider refused.

use crate::testing::TestContext;
use crate::testing::golden::{GoldenSession, Key};
use osiris_providers::github::mocks::MockGithubNetwork;
use serde_json::json;

pub async fn run(app: &TestContext, session: &mut GoldenSession<'_>) {
    let id = app.connected(28800).await;
    let github = MockGithubNetwork::on(&app.provider);
    github
        .answer_installations(&[json!({
            "id": 7,
            "account": { "login": "golden-org", "type": "Organization" },
            "repository_selection": "selected",
        })])
        .await;
    let actions = format!("/v1/connected-accounts/{id}/actions");
    session
        .send(
            "POST",
            &format!("{actions}/github.list_installations"),
            Key::Api,
            None,
            &[],
        )
        .await;
    session
        .send(
            "POST",
            &format!("{actions}/slack.list_channels"),
            Key::Api,
            None,
            &[],
        )
        .await;
}
