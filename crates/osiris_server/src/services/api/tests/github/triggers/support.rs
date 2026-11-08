//! What the trigger tests share: GitHub answering a webhook's creation and a watched pull
//! request, a trigger created and read through the API, a delivery signed as GitHub signs it, and
//! what was published and scheduled.

#![allow(
    clippy::unwrap_used,
    reason = "tests: a failure here is the test failing"
)]

use crate::testing::{API_KEY, TestContext, requests::Answer};
use axum::http::StatusCode;
use osiris_domain::EventType;
use osiris_events::catalog::events::EventMessage;
use osiris_providers::github::{
    mocks::sign_github_delivery::sign_github_delivery,
    pull_requests::mocks::generate_github_pull_request::generate_github_pull_request,
};
use serde_json::{Value, json};
use wiremock::{
    Mock, ResponseTemplate,
    matchers::{method, path},
};

/// The repository every trigger test watches.
pub const HOOKS: &str = "/repos/octo/hello/hooks";
/// The webhook GitHub creates for a trigger.
pub const HOOK: &str = "/repos/octo/hello/hooks/99";
/// The pull request a watch looks at.
pub const PULL_REQUEST: &str = "/repos/octo/hello/pulls/7";

/// Answers GitHub's webhook creation with hook 99, once.
pub async fn answer_hook_creation(app: &TestContext) {
    Mock::given(method("POST"))
        .and(path(HOOKS))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({ "id": 99 })))
        .expect(1)
        .mount(&app.provider)
        .await;
}

/// GitHub answers `verb route` with `status` and `body`, exactly `times` times.
pub async fn answer(
    app: &TestContext,
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
        .mount(&app.provider)
        .await;
}

pub async fn create(app: &TestContext, connection: &str, trigger: &str, config: Value) -> Answer {
    app.send(
        "POST",
        "/v1/triggers",
        Some(API_KEY),
        Some(json!({ "connected_account_id": connection, "trigger": trigger, "config": config })),
        &[],
    )
    .await
}

/// A connection to GitHub with a repository-event trigger on it; answers both ids.
pub async fn watching_repo_events(app: &TestContext) -> (String, String) {
    let connection = app.connected_to_github(3600).await;
    answer_hook_creation(app).await;
    let created = create(
        app,
        &connection,
        "github.on_repo_event",
        json!({ "owner": "octo", "repo": "hello", "events": ["push"] }),
    )
    .await;
    assert_eq!(created.status, StatusCode::CREATED, "{}", created.body);
    let id = created.body["id"].as_str().unwrap().to_owned();
    (connection, id)
}

/// A connection to GitHub watching pull request 7, whose first check GitHub answered; answers
/// the connection's id, the trigger's and its pending scheduled check. GitHub's answer to the
/// pull request is the test's to arrange from here.
pub async fn watching_a_pull_request(app: &TestContext) -> (String, String, String) {
    let connection = app.connected_to_github(3600).await;
    answer_hook_creation(app).await;
    let first_state = Mock::given(method("GET"))
        .and(path(PULL_REQUEST))
        .respond_with(ResponseTemplate::new(200).set_body_json(generate_github_pull_request(70, 7)))
        .mount_as_scoped(&app.provider)
        .await;
    let created = create(
        app,
        &connection,
        "github.watch_pull_request",
        json!({ "owner": "octo", "repo": "hello", "number": 7 }),
    )
    .await;
    assert_eq!(created.status, StatusCode::CREATED, "{}", created.body);
    drop(first_state);
    let id = created.body["id"].as_str().unwrap().to_owned();
    let task = pending_check(app, &id);
    (connection, id, task)
}

/// The trigger, as the API answers it.
pub async fn get_trigger(app: &TestContext, id: &str) -> Answer {
    app.send(
        "GET",
        &format!("/v1/triggers/{id}"),
        Some(API_KEY),
        None,
        &[],
    )
    .await
}

/// The secret osiris gave GitHub for the trigger's webhook, from the creation GitHub received.
pub async fn webhook_secret(app: &TestContext) -> String {
    let requests = app.provider.received_requests().await.unwrap();
    let creation = requests
        .iter()
        .find(|request| request.method.as_str() == "POST" && request.url.path() == HOOKS)
        .expect("the webhook's creation");
    let body: Value = serde_json::from_slice(&creation.body).unwrap();
    body["config"]["secret"].as_str().unwrap().to_owned()
}

/// Sends `body` to the trigger's delivery route as GitHub does, as delivery `delivery-1`, signed
/// with `secret` when given.
pub async fn deliver(
    app: &TestContext,
    trigger: &str,
    event: &str,
    body: &Value,
    secret: Option<&str>,
) -> StatusCode {
    deliver_as(app, trigger, event, body, secret, Some("delivery-1")).await
}

/// As [`deliver`], with `delivery_id` as GitHub's delivery id, or none at all.
pub async fn deliver_as(
    app: &TestContext,
    trigger: &str,
    event: &str,
    body: &Value,
    secret: Option<&str>,
    delivery_id: Option<&str>,
) -> StatusCode {
    let body = body.to_string();
    let signature = secret.map(|secret| sign_github_delivery(body.as_bytes(), secret));
    let mut headers = vec![("x-github-event", event)];
    if let Some(delivery_id) = delivery_id {
        headers.push(("x-github-delivery", delivery_id));
    }
    if let Some(signature) = signature.as_deref() {
        headers.push(("x-hub-signature-256", signature));
    }
    app.send_raw(
        "POST",
        &format!("/webhooks/triggers/{trigger}"),
        None,
        body,
        &headers,
    )
    .await
    .status
}

/// The trigger's scheduled checks still pending.
pub fn pending_checks(app: &TestContext, trigger: &str) -> Vec<String> {
    let prefix = format!("trigger-poll-{trigger}-");
    app.tasks
        .pending()
        .into_iter()
        .map(|task| task.name)
        .filter(|name| name.starts_with(&prefix))
        .collect()
}

/// The trigger's pending scheduled check.
pub fn pending_check(app: &TestContext, trigger: &str) -> String {
    pending_checks(app, trigger)
        .into_iter()
        .next()
        .expect("a scheduled check")
}

pub fn fired(app: &TestContext) -> Vec<EventMessage> {
    app.events()
        .into_iter()
        .filter(|event| event.event_type == EventType::TriggerFired)
        .collect()
}
