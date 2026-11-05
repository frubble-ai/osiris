//! State a test starts from, arranged the way a client gets there: through the API and the
//! providers' faked network.

use super::{ADMIN_KEY, API_KEY, TestContext};
use axum::http::StatusCode;
use osiris_providers::{
    github::mocks::MockGithubNetwork, linear::mocks::MockLinearNetwork,
    mocks::generate_token_answer::generate_token_answer, slack::mocks::MockSlackNetwork,
};
use serde_json::{Value, json};
use url::Url;

impl TestContext {
    /// A provider's app, registered through the admin API; answers its id.
    pub async fn auth_config(&self, provider: &str) -> String {
        let answer = self
            .send(
                "POST",
                "/v1/auth-configs",
                Some(ADMIN_KEY),
                Some(json!({ "provider": provider, "client_id": "client", "client_secret": "secret" })),
                &[],
            )
            .await;
        assert_eq!(answer.status, StatusCode::CREATED, "{}", answer.body);
        answer.body["id"].as_str().expect("an id").to_owned()
    }

    /// A GitHub app; answers its id.
    pub async fn github_config(&self) -> String {
        self.auth_config("github").await
    }

    /// Opens a consent; answers the connection's id and the `state` of its consent URL.
    pub async fn start_connect(&self, auth_config_id: &str) -> (String, String) {
        let answer = self
            .send(
                "POST",
                "/v1/connected-accounts",
                Some(API_KEY),
                Some(json!({ "auth_config_id": auth_config_id, "user_id": "org_1", "alias": "attempt-1", "callback_url": "http://app.test/integrations/callback?connection=row-1" })),
                &[],
            )
            .await;
        assert_eq!(answer.status, StatusCode::CREATED, "{}", answer.body);
        let redirect = Url::parse(answer.body["redirect_url"].as_str().expect("a consent URL"))
            .expect("a URL");
        let state = redirect
            .query_pairs()
            .find(|(name, _)| name == "state")
            .expect("a state")
            .1
            .into_owned();
        (answer.body["id"].as_str().expect("an id").to_owned(), state)
    }

    /// The provider sends the browser back with `query`; answers where the browser goes next.
    pub async fn come_back(&self, query: &str) -> String {
        let back = self
            .send("GET", &format!("/oauth/callback?{query}"), None, None, &[])
            .await;
        back.headers
            .get("location")
            .and_then(|location| location.to_str().ok())
            .unwrap_or_default()
            .to_owned()
    }

    /// A GitHub connection whose code exchange answers `answer`; answers its id.
    pub async fn connected_with(&self, answer: Value) -> String {
        MockGithubNetwork::on(&self.provider)
            .answer_code_exchange(answer)
            .await;
        let config = self.github_config().await;
        let (id, state) = self.start_connect(&config).await;
        let back = self.come_back(&format!("state={state}&code=c")).await;
        assert!(back.contains("status=active"), "{back}");
        id
    }

    /// A GitHub connection whose access token expires in `expires_in` seconds; answers its id.
    pub async fn connected(&self, expires_in: i64) -> String {
        self.connected_with(generate_token_answer(Some(expires_in)))
            .await
    }

    /// A connection to `provider` (`linear` or `slack`), whose token expires in an hour; answers its
    /// id and its access token.
    pub async fn connected_to(&self, provider: &str) -> (String, String) {
        let answer = generate_token_answer(Some(3600));
        match provider {
            "linear" => {
                MockLinearNetwork::on(&self.provider)
                    .answer_code_exchange(answer.clone())
                    .await;
            }
            "slack" => {
                MockSlackNetwork::on(&self.provider)
                    .answer_code_exchange(answer.clone())
                    .await;
            }
            other => panic!("no faked network for {other}"),
        }
        let config = self.auth_config(provider).await;
        let (id, state) = self.start_connect(&config).await;
        let back = self.come_back(&format!("state={state}&code=c")).await;
        assert!(back.contains("status=active"), "{back}");
        let token = answer["access_token"].as_str().expect("a token").to_owned();
        (id, token)
    }
}
