//! Linear state a test starts from, arranged through the API and Linear's faked network.

use super::TestContext;
use osiris_providers::{
    linear::mocks::MockLinearNetwork, mocks::generate_token_answer::generate_token_answer,
};

impl TestContext {
    /// A Linear connection granted `read`, whose token expires in an hour; answers its id and
    /// its access token.
    pub async fn connected_to_linear(&self) -> (String, String) {
        self.connected_to_linear_granting("read").await
    }

    /// A Linear connection granted `scope`, whose token expires in an hour; answers its id and its
    /// access token.
    pub async fn connected_to_linear_granting(&self, scope: &str) -> (String, String) {
        let mut answer = generate_token_answer(Some(3600));
        answer["scope"] = scope.into();
        MockLinearNetwork::on(&self.provider)
            .answer_code_exchange(answer.clone())
            .await;
        let config = self.auth_config("linear").await;
        let (id, state) = self.start_connect(&config).await;
        let back = self.come_back(&format!("state={state}&code=c")).await;
        assert!(back.contains("status=active"), "{back}");
        let token = answer["access_token"].as_str().expect("a token").to_owned();
        (id, token)
    }
}
