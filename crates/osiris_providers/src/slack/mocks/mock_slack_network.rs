//! Slack's methods, each answered as the test arranges.

use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, query_param},
};

pub struct MockSlackNetwork<'a> {
    server: &'a MockServer,
}

impl<'a> MockSlackNetwork<'a> {
    pub fn on(server: &'a MockServer) -> Self {
        Self { server }
    }

    /// The consent's code is exchanged for `answer` (Slack's user-token flow).
    pub async fn answer_code_exchange(&self, answer: Value) {
        let mut answer = answer;
        if let Some(fields) = answer.as_object_mut() {
            fields.insert("ok".into(), true.into());
        }
        Mock::given(method("POST"))
            .and(path("/api/oauth.v2.user.access"))
            .respond_with(ResponseTemplate::new(200).set_body_json(answer))
            .mount(self.server)
            .await;
    }

    /// The page of channels at `cursor` (`""`: the first), with the next cursor (`""`: the last).
    /// With `expected_calls`, the test fails unless the page is read exactly that often.
    pub async fn answer_channels_page(
        &self,
        cursor: Option<&str>,
        channels: &[Value],
        next_cursor: &str,
        expected_calls: Option<u64>,
    ) {
        let mut mock = Mock::given(method("GET")).and(path("/api/conversations.list"));
        if let Some(cursor) = cursor {
            mock = mock.and(query_param("cursor", cursor));
        }
        let mut mock = mock.respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "ok": true,
            "channels": channels,
            "response_metadata": { "next_cursor": next_cursor },
        })));
        if let Some(calls) = expected_calls {
            mock = mock.expect(calls);
        }
        mock.mount(self.server).await;
    }

    /// `conversations.list` answers 200 with `ok: false` and `error`, as Slack reports a failure.
    pub async fn answer_channels_error(&self, error: &str) {
        Mock::given(path("/api/conversations.list"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!({ "ok": false, "error": error })),
            )
            .mount(self.server)
            .await;
    }

    /// `auth.revoke` answers with `answer`.
    pub async fn answer_revoke(&self, answer: Value) {
        Mock::given(method("POST"))
            .and(path("/api/auth.revoke"))
            .respond_with(ResponseTemplate::new(200).set_body_json(answer))
            .mount(self.server)
            .await;
    }
}
