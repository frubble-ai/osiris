//! What the trigger tests share: the repository a test watches, generated for it, GitHub
//! answering a webhook's creation and a watched pull request, a trigger created and read through
//! the API, a delivery signed as GitHub signs it, and what was published and scheduled.

#![allow(
    clippy::unwrap_used,
    reason = "tests: a failure here is the test failing"
)]

use crate::testing::{API_KEY, TestContext, requests::Answer};
use axum::http::StatusCode;
use osiris_domain::EventType;
use osiris_events::catalog::events::EventMessage;
use osiris_providers::github::{
    mocks::{
        MockGithubNetwork,
        generate_github_delivery_signature::generate_github_delivery_signature,
        generate_github_names::{generate_github_login, generate_github_repo},
    },
    pull_requests::mocks::generate_github_pull_request::generate_github_pull_request,
};
use serde_json::{Value, json};

/// The repository a trigger test watches, generated for the test; GitHub's routes for it are
/// built from it.
pub struct Watched {
    pub owner: String,
    pub repo: String,
}

impl Watched {
    pub fn generate() -> Self {
        Self {
            owner: generate_github_login(),
            repo: generate_github_repo(),
        }
    }

    /// The repository's webhooks.
    pub fn hooks(&self) -> String {
        format!("/repos/{}/{}/hooks", self.owner, self.repo)
    }

    /// The webhook GitHub creates for a trigger.
    pub fn hook(&self) -> String {
        format!("{}/99", self.hooks())
    }

    /// The pull request a watch looks at.
    pub fn pull_request(&self) -> String {
        format!("/repos/{}/{}/pulls/7", self.owner, self.repo)
    }

    /// A repository-event trigger's config, watching `events`.
    pub fn repo_events(&self, events: &[&str]) -> Value {
        json!({ "owner": self.owner, "repo": self.repo, "events": events })
    }

    /// A pull request watch's config, watching pull request `number`.
    pub fn watch(&self, number: i64) -> Value {
        json!({ "owner": self.owner, "repo": self.repo, "number": number })
    }
}

/// Answers GitHub's webhook creation on `watched` with hook 99, once.
pub async fn answer_hook_creation(app: &TestContext, watched: &Watched) {
    MockGithubNetwork::on(&app.provider)
        .answer_hook_creation(&watched.owner, &watched.repo, 99)
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
    MockGithubNetwork::on(&app.provider)
        .answer_times(verb, route, status, body, times)
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

/// A connection to GitHub with a repository-event trigger on a generated repository; answers
/// both ids and the repository.
pub async fn watching_repo_events(app: &TestContext) -> (String, String, Watched) {
    let watched = Watched::generate();
    let connection = app.connected_to_github(3600).await;
    answer_hook_creation(app, &watched).await;
    let created = create(
        app,
        &connection,
        "github.on_repo_event",
        watched.repo_events(&["push"]),
    )
    .await;
    assert_eq!(created.status, StatusCode::CREATED, "{}", created.body);
    let id = created.body["id"].as_str().unwrap().to_owned();
    (connection, id, watched)
}

/// A connection to GitHub watching pull request 7 of a generated repository, whose first check
/// GitHub answered; answers the connection's id, the trigger's, its pending scheduled check and
/// the repository. GitHub's answer to the pull request is the test's to arrange from here.
pub async fn watching_a_pull_request(app: &TestContext) -> (String, String, String, Watched) {
    let connection = app.connected_to_github(3600).await;
    let (id, task, watched) = watch_a_pull_request(app, &connection).await;
    (connection, id, task, watched)
}

/// `connection` watching pull request 7 of a generated repository, whose first check GitHub
/// answered; answers the trigger's id, its pending scheduled check and the repository.
pub async fn watch_a_pull_request(
    app: &TestContext,
    connection: &str,
) -> (String, String, Watched) {
    let watched = Watched::generate();
    answer_hook_creation(app, &watched).await;
    let first_state = MockGithubNetwork::on(&app.provider)
        .answer_while(
            "GET",
            &watched.pull_request(),
            200,
            generate_github_pull_request(70, 7),
            None,
        )
        .await;
    let created = create(
        app,
        connection,
        "github.watch_pull_request",
        watched.watch(7),
    )
    .await;
    assert_eq!(created.status, StatusCode::CREATED, "{}", created.body);
    drop(first_state);
    let id = created.body["id"].as_str().unwrap().to_owned();
    let task = pending_check(app, &id);
    (id, task, watched)
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

/// The secret osiris gave GitHub for the trigger's webhook on `watched`, from the creation GitHub
/// received.
pub async fn webhook_secret(app: &TestContext, watched: &Watched) -> String {
    let hooks = watched.hooks();
    let requests = app.provider.received_requests().await.unwrap();
    let creation = requests
        .iter()
        .find(|request| request.method.as_str() == "POST" && request.url.path() == hooks)
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
    let signature =
        secret.map(|secret| generate_github_delivery_signature(body.as_bytes(), secret));
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
