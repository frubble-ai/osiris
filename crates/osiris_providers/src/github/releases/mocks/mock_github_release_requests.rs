//! GitHub's release endpoints that take a body, each answered as the test arranges only when the
//! body is the one expected.

use serde_json::Value;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, header, method, path},
};

pub struct MockGithubReleaseRequests<'a> {
    server: &'a MockServer,
}

impl<'a> MockGithubReleaseRequests<'a> {
    pub fn on(server: &'a MockServer) -> Self {
        Self { server }
    }

    /// `verb route` carrying exactly the JSON `body` and GitHub's API version answers `status`
    /// with `answer`; the test fails unless it is asked exactly once.
    pub async fn answer_with_body(
        &self,
        verb: &str,
        route: &str,
        body: Value,
        status: u16,
        answer: Value,
    ) {
        Mock::given(method(verb))
            .and(path(route))
            .and(header("x-github-api-version", "2022-11-28"))
            .and(body_json(body))
            .respond_with(ResponseTemplate::new(status).set_body_json(answer))
            .expect(1)
            .mount(self.server)
            .await;
    }
}
