//! The fake provider's endpoints, each answered as the test arranges.

use serde_json::Value;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_string_contains, header, method, path},
};

const TOKEN: &str = "/fake/oauth/token";
const REVOKE: &str = "/fake/oauth/revoke";
const WHOAMI: &str = "/fake/me";

pub struct MockFakeNetwork<'a> {
    server: &'a MockServer,
}

impl<'a> MockFakeNetwork<'a> {
    pub fn on(server: &'a MockServer) -> Self {
        Self { server }
    }

    /// The consent's code is exchanged for `answer`.
    pub async fn answer_code_exchange(&self, answer: Value) {
        self.answer_code_exchange_status(200, answer).await;
    }

    /// The consent's code exchange answers `status` with `answer`.
    pub async fn answer_code_exchange_status(&self, status: u16, answer: Value) {
        Mock::given(method("POST"))
            .and(path(TOKEN))
            .and(body_string_contains("grant_type=authorization_code"))
            .respond_with(ResponseTemplate::new(status).set_body_json(answer))
            .mount(self.server)
            .await;
    }

    /// A refresh answers `status` with `answer`.
    pub async fn answer_refresh(&self, status: u16, answer: Value) {
        Mock::given(method("POST"))
            .and(path(TOKEN))
            .and(body_string_contains("grant_type=refresh_token"))
            .respond_with(ResponseTemplate::new(status).set_body_json(answer))
            .mount(self.server)
            .await;
    }

    /// A refresh answers `status` with a body that is not JSON, as an outage does.
    pub async fn answer_refresh_text(&self, status: u16, text: &str) {
        Mock::given(method("POST"))
            .and(path(TOKEN))
            .and(body_string_contains("grant_type=refresh_token"))
            .respond_with(ResponseTemplate::new(status).set_body_string(text))
            .mount(self.server)
            .await;
    }

    /// No refresh may reach the provider; the test fails if one does.
    pub async fn expect_no_refresh(&self) {
        Mock::given(method("POST"))
            .and(path(TOKEN))
            .and(body_string_contains("grant_type=refresh_token"))
            .respond_with(ResponseTemplate::new(500))
            .expect(0)
            .mount(self.server)
            .await;
    }

    /// Ending a grant answers `status`. With `expected_calls`, the test fails unless it is asked
    /// exactly that often.
    pub async fn answer_revoke(&self, status: u16, expected_calls: Option<u64>) {
        let mut mock = Mock::given(method("POST"))
            .and(path(REVOKE))
            .respond_with(ResponseTemplate::new(status));
        if let Some(calls) = expected_calls {
            mock = mock.expect(calls);
        }
        mock.mount(self.server).await;
    }

    /// Ending a grant answers `status`, but only after `delay`.
    pub async fn answer_revoke_after(&self, delay: std::time::Duration, status: u16) {
        Mock::given(method("POST"))
            .and(path(REVOKE))
            .respond_with(ResponseTemplate::new(status).set_delay(delay))
            .mount(self.server)
            .await;
    }

    /// `fake.whoami` answers `answer` to any token.
    pub async fn answer_whoami(&self, answer: Value) {
        Mock::given(method("GET"))
            .and(path(WHOAMI))
            .respond_with(ResponseTemplate::new(200).set_body_json(answer))
            .mount(self.server)
            .await;
    }

    /// `fake.whoami` answers `answer` to `token` only.
    pub async fn answer_whoami_for_token(&self, token: &str, answer: Value) {
        Mock::given(method("GET"))
            .and(path(WHOAMI))
            .and(header("authorization", format!("Bearer {token}")))
            .respond_with(ResponseTemplate::new(200).set_body_json(answer))
            .mount(self.server)
            .await;
    }

    /// `fake.whoami` answers `status` with `text`.
    pub async fn fail_whoami(&self, status: u16, text: &str) {
        Mock::given(method("GET"))
            .and(path(WHOAMI))
            .respond_with(ResponseTemplate::new(status).set_body_string(text))
            .mount(self.server)
            .await;
    }
}
