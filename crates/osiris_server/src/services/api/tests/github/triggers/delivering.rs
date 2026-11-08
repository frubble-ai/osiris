//! Deliveries GitHub sends a trigger's webhook: each signed one published as a `trigger.fired`
//! event, an unsigned one refused as if the trigger didn't exist, and one to a failed trigger
//! taken and ignored.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use super::support::{
    Watched, answer, create, deliver, deliver_as, fired, get_trigger, watching_a_pull_request,
    watching_repo_events, webhook_secret,
};
use crate::testing::{API_KEY, setup_test_context, setup_test_context_with};
use axum::http::StatusCode;
use osiris_domain::TaskOutcome;
use osiris_providers::github::mocks::generate_sha::generate_sha;
use serde_json::json;
use sqlx::PgPool;

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn publishes_each_signed_delivery_of_a_repository_s_events(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let connection = app.connected_to_github(3600).await;
    let watched = Watched::generate();
    super::support::answer_hook_creation(&app, &watched).await;

    let created = create(
        &app,
        &connection,
        "github.on_repo_event",
        watched.repo_events(&["push"]),
    )
    .await;
    assert_eq!(created.status, StatusCode::CREATED, "{}", created.body);
    assert_eq!(created.body["status"], "active");
    assert_eq!(created.body["subscription_id"], "99");
    assert!(created.body.get("secret").is_none());
    let id = created.body["id"].as_str().unwrap();
    let secret = webhook_secret(&app, &watched).await;

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

    let retrieved = get_trigger(&app, id).await;
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
    let connection = app.connected_to_github(3600).await;
    let watched = Watched::generate();
    super::support::answer_hook_creation(&app, &watched).await;
    let created = create(
        &app,
        &connection,
        "github.on_repo_event",
        watched.repo_events(&["*"]),
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
async fn publishes_a_delivery_that_names_no_delivery_id_once(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let (_, id, watched) = watching_repo_events(&app).await;
    let secret = webhook_secret(&app, &watched).await;
    let push = json!({ "ref": "refs/heads/main", "after": generate_sha() });

    assert_eq!(
        deliver_as(&app, &id, "push", &push, Some(&secret), None).await,
        StatusCode::ACCEPTED
    );

    let events = fired(&app);
    assert_eq!(events.len(), 1, "{events:?}");
    assert_eq!(events[0].trigger_id.as_deref(), Some(id.as_str()));
    assert_eq!(
        events[0].data.as_ref().unwrap()["payload"]["ref"],
        "refs/heads/main"
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn publishes_a_delivery_too_large_to_carry_with_its_data_omitted(pool: PgPool) {
    let app = setup_test_context_with(pool, |config| {
        config.settings.triggers.max_event_data_bytes = 256;
    })
    .await;
    let (_, id, watched) = watching_repo_events(&app).await;
    let secret = webhook_secret(&app, &watched).await;
    let push = json!({ "ref": "refs/heads/main", "commits": "x".repeat(1024) });

    assert_eq!(
        deliver(&app, &id, "push", &push, Some(&secret)).await,
        StatusCode::ACCEPTED
    );

    let events = fired(&app);
    assert_eq!(events.len(), 1, "{events:?}");
    let data = events[0].data.as_ref().unwrap();
    assert_eq!(data["omitted"], true, "{data}");
    assert!(data["bytes"].as_u64().unwrap() > 256, "{data}");
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn takes_and_ignores_a_signed_delivery_to_a_failed_trigger(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let (_, id, task, watched) = watching_a_pull_request(&app).await;
    let secret = webhook_secret(&app, &watched).await;
    // GitHub no longer has the pull request: the scheduled check fails the trigger. It answers
    // that once, so the delivery below checks nothing at GitHub.
    let gone = json!({ "message": "Not Found" });
    answer(&app, "GET", &watched.pull_request(), 404, gone, 1).await;
    assert_eq!(app.run_pending(&task).await.unwrap(), TaskOutcome::Failed);
    assert_eq!(get_trigger(&app, &id).await.body["status"], "failed");

    let delivery = json!({ "action": "synchronize", "number": 7, "pull_request": { "number": 7 } });
    assert_eq!(
        deliver(&app, &id, "pull_request", &delivery, Some(&secret)).await,
        StatusCode::ACCEPTED
    );

    assert!(fired(&app).is_empty());
    assert_eq!(get_trigger(&app, &id).await.body["status"], "failed");
}
