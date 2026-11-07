//! A GitHub connection from consent to deletion: opened, the browser sent back, read, listed,
//! refreshed now, deleted with its grant ended at GitHub, and gone.

use crate::testing::TestContext;
use crate::testing::golden::{GoldenSession, Key};
use osiris_providers::github::mocks::MockGithubNetwork;
use serde_json::{Value, json};
use url::Url;

pub async fn run(app: &TestContext, session: &mut GoldenSession<'_>) {
    let github = MockGithubNetwork::on(&app.provider);
    github.answer_code_exchange(token_answer("first")).await;
    github.answer_refresh(200, token_answer("refreshed")).await;
    github.answer_revoke(204, None).await;
    let config = session
        .send(
            "POST",
            "/v1/auth-configs",
            Key::Admin,
            Some(json!({
                "provider": "github",
                "client_id": "golden-github-app",
                "client_secret": "golden-github-secret",
            })),
            &[],
        )
        .await;
    let opened = session
        .send(
            "POST",
            "/v1/connected-accounts",
            Key::Api,
            Some(json!({
                "auth_config_id": config.body["id"],
                "user_id": "org_golden",
                "alias": "golden",
                "callback_url": "https://app.example/integrations/callback",
            })),
            &[],
        )
        .await;
    let id = opened.body["id"].as_str().expect("an id").to_owned();
    let consent =
        Url::parse(opened.body["redirect_url"].as_str().expect("a consent URL")).expect("a URL");
    let state = consent
        .query_pairs()
        .find(|(name, _)| name == "state")
        .expect("a state")
        .1
        .into_owned();
    session
        .send(
            "GET",
            &format!("/oauth/callback?state={state}&code=golden"),
            Key::None,
            None,
            &[],
        )
        .await;
    let one = format!("/v1/connected-accounts/{id}");
    session.send("GET", &one, Key::Api, None, &[]).await;
    session
        .send(
            "GET",
            "/v1/connected-accounts?user_id=org_golden",
            Key::Api,
            None,
            &[],
        )
        .await;
    session
        .send("POST", &format!("{one}/refresh"), Key::Admin, None, &[])
        .await;
    session.send("DELETE", &one, Key::Api, None, &[]).await;
    session.send("GET", &one, Key::Api, None, &[]).await;
}

/// GitHub's token answer, fixed so the transcript is the same every run (tokens are never
/// answered, but the scope is).
fn token_answer(which: &str) -> Value {
    json!({
        "access_token": format!("golden-access-{which}"),
        "refresh_token": format!("golden-refresh-{which}"),
        "scope": "repo",
        "expires_in": 28800,
    })
}
