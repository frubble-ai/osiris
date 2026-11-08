//! GitHub state a test starts from, arranged through the API and GitHub's faked network.

use super::TestContext;
use osiris_providers::{
    github::mocks::MockGithubNetwork, mocks::generate_token_answer::generate_token_answer,
};
use serde_json::Value;

impl TestContext {
    /// A GitHub app; answers its id.
    pub async fn github_config(&self) -> String {
        self.auth_config("github").await
    }

    /// A GitHub connection whose code exchange answers `answer`; answers its id.
    pub async fn connected_to_github_with(&self, answer: Value) -> String {
        MockGithubNetwork::on(&self.provider)
            .answer_code_exchange(answer)
            .await;
        let config = self.github_config().await;
        let (id, state) = self.start_connect(&config).await;
        let back = self.come_back(&format!("state={state}&code=c")).await;
        assert!(back.contains("status=active"), "{back}");
        id
    }

    /// A GitHub connection whose access token expires in `expires_in` seconds, granted `repo`;
    /// answers its id.
    pub async fn connected_to_github(&self, expires_in: i64) -> String {
        let mut answer = generate_token_answer(Some(expires_in));
        answer["scope"] = "repo".into();
        self.connected_to_github_with(answer).await
    }
}
