//! GitHub's pull request endpoints that take a body, each answered as the test arranges only when
//! asked with the body the test expects.

use serde_json::Value;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, header, method, path},
};

pub struct MockPullRequestNetwork<'a> {
    server: &'a MockServer,
}

impl<'a> MockPullRequestNetwork<'a> {
    pub fn on(server: &'a MockServer) -> Self {
        Self { server }
    }

    /// `verb route` with exactly `expected` as its JSON body, carrying GitHub's API version,
    /// answers `status` with `answer`; the test fails unless it is asked exactly once.
    pub async fn answer_body(
        &self,
        verb: &str,
        route: &str,
        expected: Value,
        status: u16,
        answer: Value,
    ) {
        Mock::given(method(verb))
            .and(path(route))
            .and(header("x-github-api-version", "2022-11-28"))
            .and(body_json(expected))
            .respond_with(ResponseTemplate::new(status).set_body_json(answer))
            .expect(1)
            .mount(self.server)
            .await;
    }
}
