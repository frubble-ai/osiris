//! The checks endpoints that read a request body, each answered as the test arranges.

use crate::github::request::API_VERSION;
use serde_json::Value;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, header, method, path},
};

pub struct MockGithubChecksNetwork<'a> {
    server: &'a MockServer,
}

impl<'a> MockGithubChecksNetwork<'a> {
    pub fn on(server: &'a MockServer) -> Self {
        Self { server }
    }

    /// `verb route` carrying GitHub's API version and exactly the JSON `body` answers `status`
    /// with `answer`; the test fails unless it is asked exactly once.
    pub async fn answer_body(
        &self,
        verb: &str,
        route: &str,
        body: Value,
        status: u16,
        answer: Value,
    ) {
        Mock::given(method(verb))
            .and(path(route))
            .and(header("x-github-api-version", API_VERSION))
            .and(body_json(body))
            .respond_with(ResponseTemplate::new(status).set_body_json(answer))
            .expect(1)
            .mount(self.server)
            .await;
    }
}
