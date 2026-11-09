//! The GitHub Actions endpoints that read a request body or redirect to a file, each answered as
//! the test arranges.

use crate::github::request::API_VERSION;
use serde_json::Value;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, header, method, path},
};

pub struct MockGithubWorkflowsNetwork<'a> {
    server: &'a MockServer,
}

impl<'a> MockGithubWorkflowsNetwork<'a> {
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

    /// `GET route`, carrying GitHub's API version, redirects to `location` on the same server,
    /// where the file answers `status` with `content`. The test fails unless each is asked
    /// exactly once.
    pub async fn answer_redirect(&self, route: &str, location: &str, status: u16, content: &str) {
        Mock::given(method("GET"))
            .and(path(route))
            .and(header("x-github-api-version", API_VERSION))
            .respond_with(
                ResponseTemplate::new(302)
                    .insert_header("location", format!("{}{location}", self.server.uri())),
            )
            .expect(1)
            .mount(self.server)
            .await;
        Mock::given(method("GET"))
            .and(path(location))
            .respond_with(ResponseTemplate::new(status).set_body_string(content))
            .expect(1)
            .mount(self.server)
            .await;
    }
}
