//! A watched pull request checked on its deliveries and on schedule: a change published once, a
//! pull request GitHub no longer has failing the trigger, an outage tried again, a connection that
//! can't be used keeping the trigger waiting, and a check the trigger no longer waits for skipped.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use super::support::{
    HOOK, PULL_REQUEST, answer, deliver, fired, get_trigger, pending_check, pending_checks,
    watching_a_pull_request, webhook_secret,
};
use crate::testing::{API_KEY, setup_test_context};
use crate::use_cases::lib::tasks::next_trigger_check::next_trigger_check;
use axum::http::StatusCode;
use osiris_domain::TaskOutcome;
use osiris_providers::{
    github::{
        mocks::{MockGithubNetwork, generate_sha::generate_sha},
        pull_requests::mocks::generate_github_pull_request::generate_github_pull_request,
    },
    mocks::generate_token_answer::generate_token_answer,
};
use serde_json::json;
use sqlx::PgPool;
use time::Duration;
use wiremock::{
    Mock, ResponseTemplate,
    matchers::{method, path},
};

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn publishes_a_watched_pull_request_s_changes_once_each(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let connection = app.connected_to_github(3600).await;
    super::support::answer_hook_creation(&app).await;
    let mut pull_request = generate_github_pull_request(70, 7);
    let first_state = Mock::given(method("GET"))
        .and(path(PULL_REQUEST))
        .respond_with(ResponseTemplate::new(200).set_body_json(pull_request.clone()))
        .mount_as_scoped(&app.provider)
        .await;

    let created = super::support::create(
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
        .and(path(PULL_REQUEST))
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
async fn fails_the_trigger_when_github_no_longer_has_the_pull_request(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let (_, id, task) = watching_a_pull_request(&app).await;
    let gone = json!({ "message": "Not Found" });
    answer(&app, "GET", PULL_REQUEST, 404, gone, 1).await;

    assert_eq!(app.run_pending(&task).await.unwrap(), TaskOutcome::Failed);

    let trigger = get_trigger(&app, &id).await.body;
    assert_eq!(trigger["status"], "failed");
    assert!(trigger["last_error"].is_string(), "{trigger}");
    assert!(trigger["next_check_at"].is_null(), "{trigger}");
    assert!(pending_checks(&app, &id).is_empty());
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn tries_a_check_again_while_github_limits_the_rate(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let (_, id, name) = watching_a_pull_request(&app).await;
    let task = app.take_pending(&name);
    let before = get_trigger(&app, &id).await.body;

    // GitHub's rate limit, as a 429 and as the 403 it also answers with.
    for (status, body) in [
        (429, json!({ "message": "Too many requests" })),
        (
            403,
            json!({ "message": "API rate limit exceeded for user ID 1." }),
        ),
    ] {
        let limited = Mock::given(method("GET"))
            .and(path(PULL_REQUEST))
            .respond_with(ResponseTemplate::new(status).set_body_json(body))
            .expect(1)
            .mount_as_scoped(&app.provider)
            .await;
        assert!(
            app.run(&task).await.is_err(),
            "{status}: the task is retried"
        );
        drop(limited);
        let trigger = get_trigger(&app, &id).await.body;
        assert_eq!(trigger["status"], "active", "{status}: {trigger}");
        assert_eq!(trigger["next_check_at"], before["next_check_at"]);
        assert!(trigger["last_error"].is_null(), "{status}: {trigger}");
    }

    // Tried again once GitHub answers, the same task checks the trigger.
    answer(
        &app,
        "GET",
        PULL_REQUEST,
        200,
        generate_github_pull_request(70, 7),
        1,
    )
    .await;
    assert_eq!(app.run(&task).await.unwrap(), TaskOutcome::TriggerChecked);
    assert_eq!(pending_checks(&app, &id).len(), 1);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn keeps_the_trigger_waiting_while_its_connection_can_t_be_used(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let (connection, id, task) = watching_a_pull_request(&app).await;
    // GitHub rejects the token, and the refreshed one too: the grant is gone.
    let rejected = json!({ "message": "Bad credentials" });
    answer(&app, "GET", PULL_REQUEST, 401, rejected, 2).await;
    MockGithubNetwork::on(&app.provider)
        .answer_refresh(200, generate_token_answer(Some(28800)))
        .await;

    assert_eq!(
        app.run_pending(&task).await.unwrap(),
        TaskOutcome::TriggerChecked
    );

    assert_eq!(app.account(&connection).await["status"], "expired");
    let trigger = get_trigger(&app, &id).await.body;
    assert_eq!(trigger["status"], "active", "{trigger}");
    assert!(
        trigger["last_error"]
            .as_str()
            .unwrap()
            .contains("connection can't be used now"),
        "{trigger}"
    );
    assert!(trigger["next_check_at"].is_string(), "{trigger}");
    let next = pending_checks(&app, &id);
    assert_eq!(next.len(), 1, "{next:?}");
    assert_ne!(next[0], task);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn skips_a_check_the_trigger_no_longer_waits_for(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let (connection, id, name) = watching_a_pull_request(&app).await;
    // GitHub is never asked: a skipped check reads nothing there.
    MockGithubNetwork::on(&app.provider)
        .expect_nothing_at(PULL_REQUEST)
        .await;

    let stale = next_trigger_check(&id, &connection, 5, Duration::minutes(5));
    assert_ne!(stale.name, name);
    assert_eq!(app.run(&stale).await.unwrap(), TaskOutcome::SkippedStale);
    assert_eq!(get_trigger(&app, &id).await.body["status"], "active");
    assert_eq!(pending_checks(&app, &id), std::slice::from_ref(&name));

    // The trigger's own check, once the trigger is deleted, is skipped too.
    let task = app.take_pending(&name);
    MockGithubNetwork::on(&app.provider)
        .answer_empty("DELETE", HOOK, 204)
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
    assert_eq!(app.run(&task).await.unwrap(), TaskOutcome::SkippedStale);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn leaves_the_schedule_alone_on_a_check_a_delivery_asks_for(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let (_, id, task) = watching_a_pull_request(&app).await;
    let secret = webhook_secret(&app).await;
    let before = get_trigger(&app, &id).await.body;
    let mut pull_request = generate_github_pull_request(70, 7);
    pull_request["head"]["sha"] = json!(generate_sha());
    answer(&app, "GET", PULL_REQUEST, 200, pull_request, 1).await;

    let delivery = json!({ "action": "synchronize", "number": 7, "pull_request": { "number": 7 } });
    assert_eq!(
        deliver(&app, &id, "pull_request", &delivery, Some(&secret)).await,
        StatusCode::ACCEPTED
    );

    assert_eq!(fired(&app).len(), 1);
    let after = get_trigger(&app, &id).await.body;
    assert_eq!(after["status"], "active");
    assert_eq!(after["next_check_at"], before["next_check_at"]);
    assert_eq!(pending_checks(&app, &id), [task]);
}
