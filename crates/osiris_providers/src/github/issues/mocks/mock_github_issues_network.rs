//! GitHub's issue endpoints, each answered as the test arranges, checking the body an action sends.

use serde_json::Value;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{any, body_json, header, method, path},
};

pub struct MockGithubIssuesNetwork<'a> {
    server: &'a MockServer,
}

impl<'a> MockGithubIssuesNetwork<'a> {
    pub fn on(server: &'a MockServer) -> Self {
        Self { server }
    }

    /// `verb route` carrying GitHub's API version and exactly `sent` as its JSON body answers
    /// `status` with `answer`, or with nothing when `answer` is `None`; the test fails unless it
    /// is asked exactly once.
    pub async fn answer_body(
        &self,
        verb: &str,
        route: &str,
        sent: Value,
        status: u16,
        answer: Option<Value>,
    ) {
        let mut response = ResponseTemplate::new(status);
        if let Some(answer) = answer {
            response = response.set_body_json(answer);
        }
        Mock::given(method(verb))
            .and(path(route))
            .and(header("x-github-api-version", "2022-11-28"))
            .and(body_json(sent))
            .respond_with(response)
            .expect(1)
            .mount(self.server)
            .await;
    }

    /// `verb route` carrying GitHub's API version answers `status` with nothing; the test fails
    /// unless it is asked exactly once.
    pub async fn answer_empty(&self, verb: &str, route: &str, status: u16) {
        Mock::given(method(verb))
            .and(path(route))
            .and(header("x-github-api-version", "2022-11-28"))
            .respond_with(ResponseTemplate::new(status))
            .expect(1)
            .mount(self.server)
            .await;
    }

    /// Nothing may reach GitHub; the test fails if any request does.
    pub async fn expect_nothing(&self) {
        Mock::given(any())
            .respond_with(ResponseTemplate::new(500))
            .expect(0)
            .mount(self.server)
            .await;
    }
}
