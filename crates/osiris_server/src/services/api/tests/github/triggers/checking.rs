//! A watched pull request checked on its deliveries and on schedule: a change published once, a
//! pull request GitHub no longer has failing the trigger, an outage tried again, a connection that
//! can't be used or whose app can't mint its token keeping the trigger waiting, and a check the
//! trigger no longer waits for skipped.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests: a failure here is the test failing"
)]

use super::super::installations::{app_with_credentials, connect};
use super::support::{
    Watched, answer, deliver, fired, get_trigger, pending_check, pending_checks,
    watch_a_pull_request, watching_a_pull_request, webhook_secret,
};
use crate::testing::{ADMIN_KEY, API_KEY, setup_test_context};
use crate::use_cases::lib::tasks::next_trigger_check::next_trigger_check;
use axum::http::StatusCode;
use osiris_domain::TaskOutcome;
use osiris_providers::{
    github::{
        mocks::{
            MockGithubNetwork, generate_github_names::generate_github_token,
            generate_sha::generate_sha, mock_github_app_key::MockGithubAppKey,
        },
        pull_requests::mocks::generate_github_pull_request::generate_github_pull_request,
    },
    mocks::generate_token_answer::generate_token_answer,
};
use serde_json::json;
use sqlx::PgPool;
use time::Duration;

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn publishes_a_watched_pull_request_s_changes_once_each(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let connection = app.connected_to_github(3600).await;
    let watched = Watched::generate();
    super::support::answer_hook_creation(&app, &watched).await;
    let mut pull_request = generate_github_pull_request(70, 7);
    let first_state = MockGithubNetwork::on(&app.provider)
        .answer_while(
            "GET",
            &watched.pull_request(),
            200,
            pull_request.clone(),
            None,
        )
        .await;

    let created = super::support::create(
        &app,
        &connection,
        "github.watch_pull_request",
        watched.watch(7),
    )
    .await;
    assert_eq!(created.status, StatusCode::CREATED, "{}", created.body);
    let id = created.body["id"].as_str().unwrap().to_owned();
    let secret = webhook_secret(&app, &watched).await;
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
    let _changed = MockGithubNetwork::on(&app.provider)
        .answer_while("GET", &watched.pull_request(), 200, pull_request, None)
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
    let (_, id, task, watched) = watching_a_pull_request(&app).await;
    let gone = json!({ "message": "Not Found" });
    answer(&app, "GET", &watched.pull_request(), 404, gone, 1).await;

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
    let (_, id, name, watched) = watching_a_pull_request(&app).await;
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
        let limited = MockGithubNetwork::on(&app.provider)
            .answer_while("GET", &watched.pull_request(), status, body, Some(1))
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
        &watched.pull_request(),
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
    let (connection, id, task, watched) = watching_a_pull_request(&app).await;
    // GitHub rejects the token, and the refreshed one too: the grant is gone.
    let rejected = json!({ "message": "Bad credentials" });
    answer(&app, "GET", &watched.pull_request(), 401, rejected, 2).await;
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
async fn keeps_the_trigger_waiting_while_its_app_s_credentials_can_t_mint_a_token(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let config = app_with_credentials(&app).await;
    // The token is minted once, at connect, and the watch's first check reuses it; the test fails
    // if GitHub is asked for another.
    MockGithubNetwork::on(&app.provider)
        .answer_installation_token("77", &generate_github_token(), 1)
        .await;
    let created = connect(&app, &config, "77").await;
    assert_eq!(created.status, StatusCode::CREATED, "{}", created.body);
    let connection = created.body["id"].as_str().unwrap().to_owned();
    let (id, task, watched) = watch_a_pull_request(&app, &connection).await;
    // The app's key is replaced by one that reads as a PEM but can't sign the app's JWT, which
    // drops the token minted with the old one: the check can't mint a new one, so GitHub is never
    // asked for the pull request.
    let changed = app
        .send(
            "PATCH",
            &format!("/v1/auth-configs/{config}"),
            Some(ADMIN_KEY),
            Some(json!({
                "secret_settings": { "private_key": concat!("-----BEGIN RSA ", "PRIVATE KEY-----") },
            })),
            &[],
        )
        .await;
    assert_eq!(changed.status, StatusCode::OK, "{}", changed.body);
    MockGithubNetwork::on(&app.provider)
        .expect_nothing_at(&watched.pull_request())
        .await;

    assert_eq!(
        app.run_pending(&task).await.unwrap(),
        TaskOutcome::TriggerChecked
    );

    assert_eq!(app.account(&connection).await["status"], "active");
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
async fn keeps_the_trigger_waiting_while_github_refuses_to_mint_its_app_s_token(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let config = app_with_credentials(&app).await;
    let github = MockGithubNetwork::on(&app.provider);
    let mint = "/app/installations/77/access_tokens";
    // GitHub mints once, for connecting and the watch's first check, then refuses: the app's key
    // was rotated at GitHub.
    let first_mint = github
        .answer_while(
            "POST",
            mint,
            201,
            json!({ "token": generate_github_token(), "expires_at": "2999-01-01T00:00:00Z" }),
            Some(1),
        )
        .await;
    let created = connect(&app, &config, "77").await;
    assert_eq!(created.status, StatusCode::CREATED, "{}", created.body);
    let connection = created.body["id"].as_str().unwrap().to_owned();
    let (id, task, watched) = watch_a_pull_request(&app, &connection).await;
    drop(first_mint);
    // A new key is saved, which drops the token minted with the old one; GitHub refuses it.
    let changed = app
        .send(
            "PATCH",
            &format!("/v1/auth-configs/{config}"),
            Some(ADMIN_KEY),
            Some(
                json!({ "secret_settings": { "private_key": MockGithubAppKey::default().pem() } }),
            ),
            &[],
        )
        .await;
    assert_eq!(changed.status, StatusCode::OK, "{}", changed.body);
    github.refuse_installation_token("77", 401).await;
    github.expect_nothing_at(&watched.pull_request()).await;

    assert_eq!(
        app.run_pending(&task).await.unwrap(),
        TaskOutcome::TriggerChecked
    );

    let trigger = get_trigger(&app, &id).await.body;
    assert_eq!(trigger["status"], "active", "{trigger}");
    assert!(trigger["next_check_at"].is_string(), "{trigger}");
    assert_eq!(pending_checks(&app, &id).len(), 1);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn skips_a_check_the_trigger_no_longer_waits_for(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let (connection, id, name, watched) = watching_a_pull_request(&app).await;
    // GitHub is never asked: a skipped check reads nothing there.
    MockGithubNetwork::on(&app.provider)
        .expect_nothing_at(&watched.pull_request())
        .await;

    let stale = next_trigger_check(&id, &connection, 5, Duration::minutes(5));
    assert_ne!(stale.name, name);
    assert_eq!(app.run(&stale).await.unwrap(), TaskOutcome::SkippedStale);
    assert_eq!(get_trigger(&app, &id).await.body["status"], "active");
    assert_eq!(pending_checks(&app, &id), std::slice::from_ref(&name));

    // The trigger's own check, once the trigger is deleted, is skipped too.
    let task = app.take_pending(&name);
    MockGithubNetwork::on(&app.provider)
        .answer_empty("DELETE", &watched.hook(), 204)
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
    let (_, id, task, watched) = watching_a_pull_request(&app).await;
    let secret = webhook_secret(&app, &watched).await;
    let before = get_trigger(&app, &id).await.body;
    let mut pull_request = generate_github_pull_request(70, 7);
    pull_request["head"]["sha"] = json!(generate_sha());
    answer(&app, "GET", &watched.pull_request(), 200, pull_request, 1).await;

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
