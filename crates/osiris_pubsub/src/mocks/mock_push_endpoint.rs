//! The endpoint that receives each event as Pub/Sub would push it, answered as the test arranges.

use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

pub struct MockPushEndpoint<'a> {
    server: &'a MockServer,
}

impl<'a> MockPushEndpoint<'a> {
    pub const PATH: &'static str = "/events";

    pub fn on(server: &'a MockServer) -> Self {
        Self { server }
    }

    /// Where a push is sent.
    pub fn url(&self) -> String {
        format!("{}{}", self.server.uri(), Self::PATH)
    }

    /// Every push is answered `status`.
    pub async fn answer_push(&self, status: u16) {
        Mock::given(method("POST"))
            .and(path(Self::PATH))
            .respond_with(ResponseTemplate::new(status))
            .mount(self.server)
            .await;
    }
}
