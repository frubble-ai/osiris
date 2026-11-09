//! Slack state a test starts from, arranged through the API and Slack's faked network.

use super::TestContext;
use osiris_providers::{
    mocks::generate_token_answer::generate_token_answer, slack::mocks::MockSlackNetwork,
};

impl TestContext {
    /// A Slack connection granted `channels:read`, whose token expires in an hour; answers its id and
    /// its access token.
    pub async fn connected_to_slack(&self) -> (String, String) {
        self.connected_to_slack_granting("channels:read").await
    }

    /// A Slack connection granted `scope`, whose token expires in an hour; answers its id and its
    /// access token.
    pub async fn connected_to_slack_granting(&self, scope: &str) -> (String, String) {
        let mut answer = generate_token_answer(Some(3600));
        answer["scope"] = scope.into();
        MockSlackNetwork::on(&self.provider)
            .answer_code_exchange(answer.clone())
            .await;
        let config = self.auth_config("slack").await;
        let (id, state) = self.start_connect(&config).await;
        let back = self.come_back(&format!("state={state}&code=c")).await;
        assert!(back.contains("status=active"), "{back}");
        let token = answer["access_token"].as_str().expect("a token").to_owned();
        (id, token)
    }
}
