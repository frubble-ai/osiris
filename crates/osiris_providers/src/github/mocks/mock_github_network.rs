//! GitHub's endpoints, each answered as the test arranges.

use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_string_contains, header, method, path, path_regex, query_param},
};

pub struct MockGithubNetwork<'a> {
    server: &'a MockServer,
}

impl<'a> MockGithubNetwork<'a> {
    pub fn on(server: &'a MockServer) -> Self {
        Self { server }
    }

    /// The consent's code is exchanged for `answer`.
    pub async fn answer_code_exchange(&self, answer: Value) {
        Mock::given(method("POST"))
            .and(path("/login/oauth/access_token"))
            .and(body_string_contains("grant_type=authorization_code"))
            .respond_with(ResponseTemplate::new(200).set_body_json(answer))
            .mount(self.server)
            .await;
    }

    /// A refresh answers `status` with `answer`.
    pub async fn answer_refresh(&self, status: u16, answer: Value) {
        Mock::given(method("POST"))
            .and(path("/login/oauth/access_token"))
            .and(body_string_contains("grant_type=refresh_token"))
            .respond_with(ResponseTemplate::new(status).set_body_json(answer))
            .mount(self.server)
            .await;
    }

    /// A refresh answers `status` with a body that is not JSON, as an outage does.
    pub async fn answer_refresh_text(&self, status: u16, text: &str) {
        Mock::given(method("POST"))
            .and(path("/login/oauth/access_token"))
            .and(body_string_contains("grant_type=refresh_token"))
            .respond_with(ResponseTemplate::new(status).set_body_string(text))
            .mount(self.server)
            .await;
    }

    /// No refresh may reach GitHub; the test fails if one does.
    pub async fn expect_no_refresh(&self) {
        Mock::given(method("POST"))
            .and(path("/login/oauth/access_token"))
            .and(body_string_contains("grant_type=refresh_token"))
            .respond_with(ResponseTemplate::new(200))
            .expect(0)
            .mount(self.server)
            .await;
    }

    /// The installations the connected user can see.
    pub async fn answer_installations(&self, installations: &[Value]) {
        Mock::given(method("GET"))
            .and(path("/user/installations"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "total_count": installations.len(),
                "installations": installations,
            })))
            .mount(self.server)
            .await;
    }

    /// The installations, only for a request made with `token`.
    pub async fn answer_installations_for_token(&self, token: &str, installations: &[Value]) {
        Mock::given(method("GET"))
            .and(path("/user/installations"))
            .and(header("authorization", format!("Bearer {token}").as_str()))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "total_count": installations.len(),
                "installations": installations,
            })))
            .mount(self.server)
            .await;
    }

    /// Reading the installations answers `status` with `text`.
    pub async fn fail_installations(&self, status: u16, text: &str) {
        Mock::given(path("/user/installations"))
            .respond_with(ResponseTemplate::new(status).set_body_string(text))
            .mount(self.server)
            .await;
    }

    /// Page `page` of installation `installation`'s repositories, out of `total`. With
    /// `expected_calls`, the test fails unless the page is read exactly that often.
    pub async fn answer_repositories(
        &self,
        installation: u64,
        page: Option<u32>,
        total: usize,
        repositories: &[Value],
        expected_calls: Option<u64>,
    ) {
        let mut mock = Mock::given(method("GET")).and(path(format!(
            "/user/installations/{installation}/repositories"
        )));
        if let Some(page) = page {
            mock = mock.and(query_param("page", page.to_string()));
        }
        let mut mock = mock.respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "total_count": total,
            "repositories": repositories,
        })));
        if let Some(calls) = expected_calls {
            mock = mock.expect(calls);
        }
        mock.mount(self.server).await;
    }

    /// Ending a grant answers `status`. With `expected_calls`, the test fails unless it is asked
    /// exactly that often.
    pub async fn answer_revoke(&self, status: u16, expected_calls: Option<u64>) {
        let mut mock = Mock::given(method("DELETE"))
            .and(path_regex("^/applications/.*/grant$"))
            .respond_with(ResponseTemplate::new(status));
        if let Some(calls) = expected_calls {
            mock = mock.expect(calls);
        }
        mock.mount(self.server).await;
    }

    /// Ending a grant answers `status`, but only after `delay`.
    pub async fn answer_revoke_after(&self, delay: std::time::Duration, status: u16) {
        Mock::given(method("DELETE"))
            .and(path_regex("^/applications/.*/grant$"))
            .respond_with(ResponseTemplate::new(status).set_delay(delay))
            .mount(self.server)
            .await;
    }
}
