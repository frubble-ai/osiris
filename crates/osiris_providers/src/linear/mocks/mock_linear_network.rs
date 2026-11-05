//! Linear's endpoints, each answered as the test arranges.

use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_partial_json, header, method, path},
};

pub struct MockLinearNetwork<'a> {
    server: &'a MockServer,
}

impl<'a> MockLinearNetwork<'a> {
    pub fn on(server: &'a MockServer) -> Self {
        Self { server }
    }

    /// The consent's code is exchanged for `answer`.
    pub async fn answer_code_exchange(&self, answer: Value) {
        Mock::given(method("POST"))
            .and(path("/oauth/token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(answer))
            .mount(self.server)
            .await;
    }

    /// The page of teams after `after` (`None`: the first), with the cursor of the next page.
    pub async fn answer_teams_page(
        &self,
        after: Option<&str>,
        teams: &[Value],
        next: Option<&str>,
        expected_calls: Option<u64>,
    ) {
        let mock = Mock::given(method("POST"))
            .and(path("/graphql"))
            .and(body_partial_json(
                json!({ "variables": { "after": after } }),
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!({ "data": { "teams": {
                "nodes": teams,
                "pageInfo": { "hasNextPage": next.is_some(), "endCursor": next },
            } } })),
            );
        match expected_calls {
            Some(calls) => mock.expect(calls),
            None => mock,
        }
        .mount(self.server)
        .await;
    }

    /// Ending a grant answers `status`. With `expected_calls`, the test fails unless it is asked
    /// exactly that often.
    pub async fn answer_revoke(&self, status: u16, expected_calls: Option<u64>) {
        let mock = Mock::given(method("POST"))
            .and(path("/oauth/revoke"))
            .respond_with(ResponseTemplate::new(status));
        match expected_calls {
            Some(calls) => mock.expect(calls),
            None => mock,
        }
        .mount(self.server)
        .await;
    }

    /// Every team on one page, only for a request made with `token`.
    pub async fn answer_teams_for_token(&self, token: &str, teams: &[Value]) {
        Mock::given(method("POST"))
            .and(path("/graphql"))
            .and(header("authorization", format!("Bearer {token}").as_str()))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!({ "data": { "teams": {
                "nodes": teams,
                "pageInfo": { "hasNextPage": false, "endCursor": null },
            } } })),
            )
            .mount(self.server)
            .await;
    }

    /// Every GraphQL request answers 200 with `errors`, as Linear reports a failure.
    pub async fn answer_graphql_errors(&self, errors: Value) {
        Mock::given(path("/graphql"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "errors": errors })))
            .mount(self.server)
            .await;
    }
}
