//! Osiris's API as the client sees it: an endpoint answers only a call that carries the expected
//! key and the version header, so an unmatched call fails the test.

use osiris_api_models::version::{CURRENT, HEADER};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path},
};

pub struct MockOsirisApi<'a> {
    server: &'a MockServer,
}

impl<'a> MockOsirisApi<'a> {
    pub fn on(server: &'a MockServer) -> Self {
        Self { server }
    }

    /// `verb path` called with `key` answers `status` with `body`.
    pub async fn answer(&self, verb: &str, route: &str, key: &str, status: u16, body: &str) {
        Mock::given(method(verb))
            .and(path(route))
            .and(header("authorization", format!("Bearer {key}").as_str()))
            .and(header(HEADER, CURRENT))
            .respond_with(ResponseTemplate::new(status).set_body_string(body))
            .mount(self.server)
            .await;
    }
}
