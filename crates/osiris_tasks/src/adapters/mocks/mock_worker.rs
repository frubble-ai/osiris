//! The worker's `POST /tasks`, answered as the test arranges.

use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

pub struct MockWorker<'a> {
    server: &'a MockServer,
}

impl<'a> MockWorker<'a> {
    pub const PATH: &'static str = "/tasks";

    pub fn on(server: &'a MockServer) -> Self {
        Self { server }
    }

    /// Where a task is sent.
    pub fn url(&self) -> String {
        format!("{}{}", self.server.uri(), Self::PATH)
    }

    /// The next task is answered `status`, once.
    pub async fn answer_once(&self, status: u16) {
        Mock::given(method("POST"))
            .and(path(Self::PATH))
            .respond_with(ResponseTemplate::new(status))
            .up_to_n_times(1)
            .mount(self.server)
            .await;
    }

    /// The tasks sent so far, with their `Authorization` header.
    pub async fn received(&self) -> Vec<(Option<String>, serde_json::Value)> {
        self.server
            .received_requests()
            .await
            .unwrap_or_default()
            .iter()
            .map(|request| {
                (
                    request
                        .headers
                        .get("authorization")
                        .map(|value| value.to_str().unwrap_or_default().to_owned()),
                    serde_json::from_slice(&request.body).unwrap_or_default(),
                )
            })
            .collect()
    }
}
