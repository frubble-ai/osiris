//! GitHub's endpoints, each answered as the test arranges.

use serde_json::{Value, json};
use wiremock::{
    Mock, MockGuard, MockServer, ResponseTemplate,
    matchers::{body_json, body_string_contains, header, method, path, path_regex, query_param},
};

pub struct MockGithubNetwork<'a> {
    server: &'a MockServer,
}

impl<'a> MockGithubNetwork<'a> {
    pub fn on(server: &'a MockServer) -> Self {
        Self { server }
    }

    /// `verb route` with every `(name, value)` of `query`, carrying GitHub's API version, answers
    /// `status` with `body`; the test fails unless it is asked exactly once.
    pub async fn answer(
        &self,
        verb: &str,
        route: &str,
        query: &[(&str, &str)],
        status: u16,
        body: Value,
    ) {
        let mut mock = Mock::given(method(verb))
            .and(path(route))
            .and(header("x-github-api-version", "2022-11-28"));
        for (name, value) in query {
            mock = mock.and(query_param(*name, *value));
        }
        mock.respond_with(ResponseTemplate::new(status).set_body_json(body))
            .expect(1)
            .mount(self.server)
            .await;
    }

    /// As [`Self::answer`], and only for a request whose JSON body is `request`: the test fails
    /// unless GitHub is sent exactly that body, once.
    pub async fn answer_with_body(
        &self,
        verb: &str,
        route: &str,
        request: Value,
        status: u16,
        body: Value,
    ) {
        Mock::given(method(verb))
            .and(path(route))
            .and(header("x-github-api-version", "2022-11-28"))
            .and(body_json(request))
            .respond_with(ResponseTemplate::new(status).set_body_json(body))
            .expect(1)
            .mount(self.server)
            .await;
    }

    /// `verb route` answers `status` with no body at all, as GitHub's 204 does; the test fails
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

    /// `verb route` answers `status` with `body`; the test fails unless it is asked exactly
    /// `times` times.
    pub async fn answer_times(
        &self,
        verb: &str,
        route: &str,
        status: u16,
        body: Value,
        times: u64,
    ) {
        Mock::given(method(verb))
            .and(path(route))
            .respond_with(ResponseTemplate::new(status).set_body_json(body))
            .expect(times)
            .mount(self.server)
            .await;
    }

    /// `verb route` answers `status` with `body` to its first request only; later ones go to the
    /// next arrangement.
    pub async fn answer_once(&self, verb: &str, route: &str, status: u16, body: Value) {
        Mock::given(method(verb))
            .and(path(route))
            .respond_with(ResponseTemplate::new(status).set_body_json(body))
            .up_to_n_times(1)
            .mount(self.server)
            .await;
    }

    /// `verb route` answers `status` with `body` for as long as the guard it answers is kept; with
    /// `times`, the test fails unless it is asked exactly that often meanwhile.
    pub async fn answer_while(
        &self,
        verb: &str,
        route: &str,
        status: u16,
        body: Value,
        times: Option<u64>,
    ) -> MockGuard {
        let mut mock = Mock::given(method(verb))
            .and(path(route))
            .respond_with(ResponseTemplate::new(status).set_body_json(body));
        if let Some(times) = times {
            mock = mock.expect(times);
        }
        mock.mount_as_scoped(self.server).await
    }

    /// `GET route` answers `body` only to requests made with `token`; the test fails unless it
    /// is asked exactly `times` times.
    pub async fn answer_for_token(&self, route: &str, token: &str, body: Value, times: u64) {
        Mock::given(method("GET"))
            .and(path(route))
            .and(header("authorization", format!("Bearer {token}")))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .expect(times)
            .mount(self.server)
            .await;
    }

    /// Creating a webhook on `owner/repo` answers hook `id`, once.
    pub async fn answer_hook_creation(&self, owner: &str, repo: &str, id: u64) {
        self.answer_times(
            "POST",
            &format!("/repos/{owner}/{repo}/hooks"),
            201,
            json!({ "id": id }),
            1,
        )
        .await;
    }

    /// No request may reach `route`; the test fails if one does.
    pub async fn expect_nothing_at(&self, route: &str) {
        Mock::given(path(route))
            .respond_with(ResponseTemplate::new(500))
            .expect(0)
            .mount(self.server)
            .await;
    }

    /// Minting a token for installation `installation` answers `token`, good for an hour; the
    /// test fails unless it is minted exactly `expected_calls` times.
    pub async fn answer_installation_token(
        &self,
        installation: &str,
        token: &str,
        expected_calls: u64,
    ) {
        Mock::given(method("POST"))
            .and(path(format!(
                "/app/installations/{installation}/access_tokens"
            )))
            .respond_with(ResponseTemplate::new(201).set_body_json(json!({
                "token": token,
                "expires_at": "2999-01-01T00:00:00Z",
                "permissions": { "contents": "read", "metadata": "read" },
            })))
            .expect(expected_calls)
            .mount(self.server)
            .await;
    }

    /// Minting a token for installation `installation` is refused with `status`.
    pub async fn refuse_installation_token(&self, installation: &str, status: u16) {
        Mock::given(method("POST"))
            .and(path(format!(
                "/app/installations/{installation}/access_tokens"
            )))
            .respond_with(
                ResponseTemplate::new(status).set_body_json(json!({ "message": "Not Found" })),
            )
            .mount(self.server)
            .await;
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

    /// The consent's code is refused: the exchange answers `status` with `answer`.
    pub async fn answer_code_exchange_status(&self, status: u16, answer: Value) {
        Mock::given(method("POST"))
            .and(path("/login/oauth/access_token"))
            .and(body_string_contains("grant_type=authorization_code"))
            .respond_with(ResponseTemplate::new(status).set_body_json(answer))
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
