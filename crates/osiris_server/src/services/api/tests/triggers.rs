//! Triggers through the API: created at GitHub as a repository webhook delivering to osiris, each
//! signed delivery published as a `trigger.fired` event, an unsigned one refused as if the trigger
//! didn't exist, a watched pull request checked on its deliveries and on schedule, and the webhook
//! removed when the trigger is deleted.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use crate::testing::{API_KEY, TestContext, requests::Answer, setup_test_context};
use axum::http::StatusCode;
use osiris_domain::{EventType, TaskOutcome};
use osiris_events::catalog::events::EventMessage;
use osiris_providers::github::{
    mocks::{
        MockGithubNetwork, generate_sha::generate_sha, sign_github_delivery::sign_github_delivery,
    },
    pull_requests::mocks::generate_github_pull_request::generate_github_pull_request,
};
use serde_json::{Value, json};
use sqlx::PgPool;
use wiremock::{
    Mock, ResponseTemplate,
    matchers::{method, path},
};

/// Answers GitHub's webhook creation with hook 99, once.
async fn answer_hook_creation(app: &TestContext) {
    Mock::given(method("POST"))
        .and(path("/repos/octo/hello/hooks"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({ "id": 99 })))
        .expect(1)
        .mount(&app.provider)
        .await;
}

async fn create(app: &TestContext, connection: &str, trigger: &str, config: Value) -> Answer {
    app.send(
        "POST",
        "/v1/triggers",
        Some(API_KEY),
        Some(json!({ "connected_account_id": connection, "trigger": trigger, "config": config })),
        &[],
    )
    .await
}

/// The secret osiris gave GitHub for the trigger's webhook, from the creation GitHub received.
async fn webhook_secret(app: &TestContext) -> String {
    let requests = app.provider.received_requests().await.unwrap();
    let creation = requests
        .iter()
        .find(|request| {
            request.method.as_str() == "POST" && request.url.path() == "/repos/octo/hello/hooks"
        })
        .expect("the webhook's creation");
    let body: Value = serde_json::from_slice(&creation.body).unwrap();
    body["config"]["secret"].as_str().unwrap().to_owned()
}

/// Sends `body` to the trigger's delivery route as GitHub does, signed with `secret` when given.
async fn deliver(
    app: &TestContext,
    trigger: &str,
    event: &str,
    body: &Value,
    secret: Option<&str>,
) -> StatusCode {
    let body = body.to_string();
    let signature = secret.map(|secret| sign_github_delivery(body.as_bytes(), secret));
    let mut headers = vec![
        ("x-github-event", event),
        ("x-github-delivery", "delivery-1"),
    ];
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

/// The trigger's pending scheduled check.
fn pending_check(app: &TestContext, trigger: &str) -> String {
    let prefix = format!("trigger-poll-{trigger}-");
    app.tasks
        .pending()
        .into_iter()
        .map(|task| task.name)
        .find(|name| name.starts_with(&prefix))
        .expect("a scheduled check")
}

fn fired(app: &TestContext) -> Vec<EventMessage> {
    app.events()
        .into_iter()
        .filter(|event| event.event_type == EventType::TriggerFired)
        .collect()
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn publishes_each_signed_delivery_of_a_repository_s_events(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let connection = app.connected(3600).await;
    answer_hook_creation(&app).await;

    let created = create(
        &app,
        &connection,
        "github.on_repo_event",
        json!({ "owner": "octo", "repo": "hello", "events": ["push"] }),
    )
    .await;
    assert_eq!(created.status, StatusCode::CREATED, "{}", created.body);
    assert_eq!(created.body["status"], "active");
    assert_eq!(created.body["subscription_id"], "99");
    assert!(created.body.get("secret").is_none());
    let id = created.body["id"].as_str().unwrap();
    let secret = webhook_secret(&app).await;

    let push = json!({ "ref": "refs/heads/main", "after": generate_sha() });
    assert_eq!(
        deliver(&app, id, "push", &push, Some(&secret)).await,
        StatusCode::ACCEPTED
    );
    // GitHub delivering the same delivery again publishes it under the same event id.
    assert_eq!(
        deliver(&app, id, "push", &push, Some(&secret)).await,
        StatusCode::ACCEPTED
    );
    // An event the trigger doesn't take is taken and ignored.
    assert_eq!(
        deliver(&app, id, "issues", &json!({}), Some(&secret)).await,
        StatusCode::ACCEPTED
    );

    let events = fired(&app);
    assert_eq!(events.len(), 2, "{events:?}");
    assert_eq!(events[0].event_id, events[1].event_id);
    assert_eq!(events[0].trigger_id.as_deref(), Some(id));
    assert_eq!(events[0].trigger.as_deref(), Some("github.on_repo_event"));
    assert_eq!(events[0].connected_account_id, connection);
    assert_eq!(
        events[0].data.as_ref().unwrap()["payload"]["ref"],
        "refs/heads/main"
    );

    let retrieved = app
        .send(
            "GET",
            &format!("/v1/triggers/{id}"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(retrieved.status, StatusCode::OK, "{}", retrieved.body);
    let listed = app
        .send(
            "GET",
            &format!("/v1/triggers?connected_account_id={connection}"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(listed.status, StatusCode::OK, "{}", listed.body);
    assert_eq!(listed.body["data"][0]["id"], id);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_a_delivery_not_signed_with_the_trigger_s_secret_as_if_it_did_not_exist(
    pool: PgPool,
) {
    let app = setup_test_context(pool).await;
    let connection = app.connected(3600).await;
    answer_hook_creation(&app).await;
    let created = create(
        &app,
        &connection,
        "github.on_repo_event",
        json!({ "owner": "octo", "repo": "hello", "events": ["*"] }),
    )
    .await;
    let id = created.body["id"].as_str().unwrap();
    let push = json!({ "ref": "refs/heads/main" });

    assert_eq!(
        deliver(&app, id, "push", &push, None).await,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        deliver(&app, id, "push", &push, Some("a-guessed-secret")).await,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        deliver(&app, "tr_missing", "push", &push, Some("a-guessed-secret")).await,
        StatusCode::NOT_FOUND
    );
    assert!(fired(&app).is_empty());
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_a_config_the_provider_cannot_watch(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let connection = app.connected(3600).await;

    for (trigger, config) in [
        (
            "github.on_repo_event",
            json!({ "owner": "octo", "repo": "hello", "events": ["pushes"] }),
        ),
        (
            "github.watch_pull_request",
            json!({ "owner": "octo", "repo": "hello", "number": 0 }),
        ),
        ("github.on_nothing", json!({})),
    ] {
        let refused = create(&app, &connection, trigger, config).await;
        assert_eq!(
            refused.status,
            StatusCode::BAD_REQUEST,
            "{trigger}: {}",
            refused.body
        );
        assert_eq!(refused.body["error"]["code"], "OS_02");
    }
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn publishes_a_watched_pull_request_s_changes_once_each(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let connection = app.connected(3600).await;
    answer_hook_creation(&app).await;
    let mut pull_request = generate_github_pull_request(70, 7);
    let first_state = Mock::given(method("GET"))
        .and(path("/repos/octo/hello/pulls/7"))
        .respond_with(ResponseTemplate::new(200).set_body_json(pull_request.clone()))
        .mount_as_scoped(&app.provider)
        .await;

    let created = create(
        &app,
        &connection,
        "github.watch_pull_request",
        json!({ "owner": "octo", "repo": "hello", "number": 7 }),
    )
    .await;
    assert_eq!(created.status, StatusCode::CREATED, "{}", created.body);
    let id = created.body["id"].as_str().unwrap().to_owned();
    let secret = webhook_secret(&app).await;
    let task = pending_check(&app, &id);
    assert!(created.body["next_check_at"].is_string());
    // The first check recorded the state the watch starts from, firing nothing; the same state
    // checked again on schedule fires nothing either.
    assert_eq!(
        app.run_pending(&task).await.unwrap(),
        TaskOutcome::TriggerChecked
    );
    assert!(fired(&app).is_empty());
    drop(first_state);

    pull_request["head"]["sha"] = json!(generate_sha());
    Mock::given(method("GET"))
        .and(path("/repos/octo/hello/pulls/7"))
        .respond_with(ResponseTemplate::new(200).set_body_json(pull_request))
        .mount(&app.provider)
        .await;
    // A delivery about the pull request asks for a check at once; the next scheduled check sees
    // nothing new.
    let delivery = json!({ "action": "synchronize", "number": 7, "pull_request": { "number": 7 } });
    assert_eq!(
        deliver(&app, &id, "pull_request", &delivery, Some(&secret)).await,
        StatusCode::ACCEPTED
    );
    let next = pending_check(&app, &id);
    assert_eq!(
        app.run_pending(&next).await.unwrap(),
        TaskOutcome::TriggerChecked
    );

    let events = fired(&app);
    assert_eq!(events.len(), 1, "{events:?}");
    assert_eq!(
        events[0].data.as_ref().unwrap()["pull_request"]["number"],
        7
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn removes_the_webhook_when_the_trigger_is_deleted(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let connection = app.connected(3600).await;
    answer_hook_creation(&app).await;
    let created = create(
        &app,
        &connection,
        "github.on_repo_event",
        json!({ "owner": "octo", "repo": "hello", "events": ["push"] }),
    )
    .await;
    let id = created.body["id"].as_str().unwrap();
    MockGithubNetwork::on(&app.provider)
        .answer_empty("DELETE", "/repos/octo/hello/hooks/99", 204)
        .await;

    let deleted = app
        .send(
            "DELETE",
            &format!("/v1/triggers/{id}"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;

    assert_eq!(deleted.status, StatusCode::OK, "{}", deleted.body);
    assert_eq!(deleted.body["deleted"], true);
    let gone = app
        .send(
            "GET",
            &format!("/v1/triggers/{id}"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(gone.status, StatusCode::NOT_FOUND);
}
