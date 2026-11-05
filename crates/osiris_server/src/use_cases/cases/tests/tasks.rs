//! Running scheduled tasks: token refreshes (success, out-of-date tasks, temporary and permanent
//! failures), consent expiries and event retries.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::panic,
    reason = "tests: a failure here is the test failing"
)]

use crate::testing::{API_KEY, setup_test_context};
use axum::http::StatusCode;
use osiris_domain::EventType;
use osiris_providers::github::mocks::MockGithubNetwork;
use osiris_tasks::{RefreshReason, Task, TaskPayload, names};
use serde_json::json;
use sqlx::PgPool;

/// Makes the provider answer a refresh with `status` and `body`.
async fn refresh_answers(app: &crate::testing::TestContext, status: u16, body: serde_json::Value) {
    MockGithubNetwork::on(&app.provider)
        .answer_refresh(status, body)
        .await;
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn refreshes_saves_the_next_task_with_the_new_tokens_then_acknowledges(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected(28800).await;
    refresh_answers(
        &app,
        200,
        json!({ "access_token": "ghu_2", "refresh_token": "ghr_2", "expires_in": 28800 }),
    )
    .await;

    assert!(app.run_pending(&names::refresh(&id, 1)).await.is_ok());

    // By the time the worker acknowledged, the next task existed and was saved with the tokens.
    let next = names::refresh(&id, 2);
    let task = app.tasks.get(&next).expect("the next refresh is scheduled");
    let TaskPayload::ConnectionRefresh(refresh) = &task.payload else {
        panic!("a refresh task");
    };
    assert_eq!(refresh.generation, 2);
    assert_eq!(refresh.reason, RefreshReason::Expiry);
    let stored = app
        .use_cases
        .context()
        .store
        .connected_account(&id)
        .await
        .unwrap();
    assert_eq!(stored.token_generation, 2);
    assert_eq!(stored.refresh_task.as_deref(), Some(next.as_str()));
    assert_eq!(app.tasks.pending().len(), 1, "exactly one pending refresh");
    assert_eq!(app.outcomes(&id).await, ["refreshed"]);
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn schedules_a_keepalive_for_a_token_that_does_not_expire(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected(28800).await;
    refresh_answers(
        &app,
        200,
        json!({ "access_token": "ghu_2", "refresh_token": "ghr_2" }),
    )
    .await;

    assert!(app.run_pending(&names::refresh(&id, 1)).await.is_ok());
    let task = app.tasks.get(&names::refresh(&id, 2)).unwrap();
    let TaskPayload::ConnectionRefresh(refresh) = &task.payload else {
        panic!("a refresh task");
    };
    assert_eq!(refresh.reason, RefreshReason::Keepalive);
    let ahead = task.run_at - time::OffsetDateTime::now_utc();
    assert!(
        ahead > time::Duration::days(28) && ahead <= time::Duration::days(29),
        "{ahead}"
    );
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn acknowledges_a_task_for_an_older_generation_as_stale_and_does_nothing(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected(28800).await;
    // The provider must not be asked to refresh: the test fails if it is.
    MockGithubNetwork::on(&app.provider)
        .expect_no_refresh()
        .await;
    let stale = Task {
        name: names::refresh(&id, 0),
        run_at: time::OffsetDateTime::now_utc(),
        payload: TaskPayload::ConnectionRefresh(osiris_tasks::RefreshTask {
            tenant_id: "org_1".into(),
            connected_account_id: id.clone(),
            generation: 0,
            reason: RefreshReason::Expiry,
        }),
    };

    assert!(app.run(&stale).await.is_ok());
    let stored = app
        .use_cases
        .context()
        .store
        .connected_account(&id)
        .await
        .unwrap();
    assert_eq!(stored.token_generation, 1, "nothing was refreshed");
    assert_eq!(app.outcomes(&id).await, ["skipped_stale"]);
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn acknowledges_a_refresh_for_a_deleted_connection_as_stale(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected(28800).await;
    let task = app.tasks.take(&names::refresh(&id, 1)).unwrap();
    sqlx::query("DELETE FROM connected_account WHERE id = $1")
        .bind(&id)
        .execute(&app.use_cases.context().store.pool)
        .await
        .unwrap();
    assert!(app.run(&task).await.is_ok());
    assert_eq!(app.outcomes(&id).await, ["skipped_stale"]);
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn answers_an_error_on_a_temporary_failure_so_the_same_task_runs_again(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected(28800).await;
    MockGithubNetwork::on(&app.provider)
        .answer_refresh_text(503, "Service Unavailable")
        .await;
    let task = app.tasks.take(&names::refresh(&id, 1)).unwrap();

    assert!(app.run(&task).await.is_err());
    let stored = app
        .use_cases
        .context()
        .store
        .connected_account(&id)
        .await
        .unwrap();
    assert_eq!(stored.token_generation, 1, "the tokens are kept");
    assert_eq!(stored.refresh_task.as_deref(), Some(task.name.as_str()));
    assert_eq!(stored.refresh_failures, 1);
    assert!(app.tasks.pending().is_empty(), "nothing new is scheduled");
    assert_eq!(app.outcomes(&id).await, ["failed"]);
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn keeps_new_tokens_disabled_when_the_next_task_cannot_be_scheduled_then_restores_on_retry(
    pool: PgPool,
) {
    let app = setup_test_context(pool).await;
    let id = app.connected(28800).await;
    refresh_answers(
        &app,
        200,
        json!({ "access_token": "ghu_2", "refresh_token": "ghr_2", "expires_in": 28800 }),
    )
    .await;
    let task = app.tasks.take(&names::refresh(&id, 1)).unwrap();
    app.tasks.fail(true);

    // The refresh worked but its next task could not be scheduled: the new tokens are kept (the
    // provider may already have invalidated the old refresh token) and the connection is disabled.
    assert!(app.run(&task).await.is_err());
    let stored = app
        .use_cases
        .context()
        .store
        .connected_account(&id)
        .await
        .unwrap();
    assert_eq!(stored.token_generation, 2);
    assert!(stored.refresh_task.is_none());
    let account = app.account(&id).await;
    assert_eq!(account["status"], "disabled");
    assert_eq!(account["setup_complete"], false);

    // Cloud Tasks retries the same task once the scheduler works again: the refresh is scheduled
    // for the new tokens and the connection is active again, without asking the provider again.
    app.tasks.fail(false);
    assert!(app.run(&task).await.is_ok());
    let account = app.account(&id).await;
    assert_eq!(account["status"], "active");
    assert_eq!(account["refresh_task"], names::refresh(&id, 2).as_str());
    assert!(app.tasks.get(&names::refresh(&id, 2)).is_some());
    assert_eq!(app.outcomes(&id).await, ["failed", "rescheduled"]);
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn restores_a_disabled_connection_when_an_action_uses_it(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected(28800).await;
    refresh_answers(
        &app,
        200,
        json!({ "access_token": "ghu_2", "refresh_token": "ghr_2", "expires_in": 28800 }),
    )
    .await;
    let task = app.tasks.take(&names::refresh(&id, 1)).unwrap();
    app.tasks.fail(true);
    let _ = app.run(&task).await;
    app.tasks.fail(false);
    MockGithubNetwork::on(&app.provider)
        .answer_installations(&[])
        .await;

    let answer = app
        .send(
            "POST",
            &format!("/v1/connected-accounts/{id}/actions/github.list_installations"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(answer.status, StatusCode::OK, "{}", answer.body);
    assert_eq!(app.account(&id).await["status"], "active");
    assert!(app.tasks.get(&names::refresh(&id, 2)).is_some());
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn ends_the_connection_on_a_permanent_refusal_with_its_event_and_no_new_task(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected(28800).await;
    refresh_answers(&app, 400, json!({ "error": "invalid_grant" })).await;

    assert!(app.run_pending(&names::refresh(&id, 1)).await.is_ok());
    let account = app.account(&id).await;
    assert_eq!(account["status"], "expired");
    assert_eq!(account["setup_complete"], false);
    assert!(account["refresh_task"].is_null());
    assert!(app.tasks.pending().is_empty(), "nothing is rescheduled");
    assert!(
        app.events()
            .iter()
            .any(|event| event.event_type == EventType::ConnectionExpired)
    );
    assert_eq!(app.outcomes(&id).await, ["failed"]);
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn deletes_the_pending_refresh_task_on_disconnect(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected(28800).await;
    MockGithubNetwork::on(&app.provider)
        .answer_revoke(204, None)
        .await;

    let answer = app
        .send(
            "DELETE",
            &format!("/v1/connected-accounts/{id}"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(answer.status, StatusCode::OK, "{}", answer.body);
    assert!(app.tasks.cancelled().contains(&names::refresh(&id, 1)));
    assert!(app.tasks.get(&names::refresh(&id, 1)).is_none());
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn fails_a_consent_still_open_when_its_expiry_task_runs(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let config = app.github_config().await;
    let (id, _) = app.start_connect(&config).await;

    assert!(app.run_pending(&names::consent(&id)).await.is_ok());
    assert_eq!(app.account(&id).await["status"], "failed");
    assert_eq!(app.outcomes(&id).await, ["consent_expired"]);
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn acknowledges_the_expiry_of_a_finished_consent_as_stale(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected(28800).await;
    let expiry = Task {
        name: names::consent(&id),
        run_at: time::OffsetDateTime::now_utc(),
        payload: TaskPayload::ConsentExpire(osiris_tasks::ConsentTask {
            tenant_id: "org_1".into(),
            connected_account_id: id.clone(),
        }),
    };
    assert!(app.run(&expiry).await.is_ok());
    assert_eq!(app.account(&id).await["status"], "active");
    assert_eq!(app.outcomes(&id).await, ["skipped_stale"]);
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn publishes_an_event_again_through_a_task_named_after_it(pool: PgPool) {
    let app = setup_test_context(pool).await;
    app.bus.fail(true);
    let id = app.connected(28800).await;
    assert!(app.events().is_empty());
    let retry = app
        .tasks
        .pending()
        .into_iter()
        .find(|task| matches!(task.payload, TaskPayload::EventPublish(_)))
        .expect("the event's retry is scheduled");
    let TaskPayload::EventPublish(event) = &retry.payload else {
        panic!("an event task");
    };
    assert_eq!(retry.name, names::event(&event.event_id));

    // While the bus still fails, the worker answers an error so the task runs again.
    assert!(app.run(&retry).await.is_err());
    app.bus.fail(false);
    assert!(app.run(&retry).await.is_ok());
    let events = app.events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].event_id, event.event_id);
    assert_eq!(events[0].event_type, EventType::ConnectionActive);
    assert_eq!(app.outcomes(&id).await, ["failed", "published"]);
}

#[sqlx::test(migrator = "osiris_storage::MIGRATOR")]
async fn refreshes_now_through_a_task_that_replaces_the_scheduled_one(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let id = app.connected(28800).await;
    refresh_answers(
        &app,
        200,
        json!({ "access_token": "ghu_2", "refresh_token": "ghr_2", "expires_in": 28800 }),
    )
    .await;

    let answer = app
        .send(
            "POST",
            &format!("/v1/connected-accounts/{id}/refresh"),
            Some(crate::testing::ADMIN_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(answer.status, StatusCode::ACCEPTED, "{}", answer.body);
    let now = names::refresh_now(&id, 1);
    assert_eq!(answer.body["refresh_task"], now.as_str());
    assert!(app.tasks.cancelled().contains(&names::refresh(&id, 1)));
    assert_eq!(
        app.tasks.pending().len(),
        1,
        "still exactly one pending refresh"
    );

    assert!(app.run_pending(&now).await.is_ok());
    assert!(app.tasks.get(&names::refresh(&id, 2)).is_some());

    let with_api_key = app
        .send(
            "POST",
            &format!("/v1/connected-accounts/{id}/refresh"),
            Some(API_KEY),
            None,
            &[],
        )
        .await;
    assert_eq!(
        with_api_key.status,
        StatusCode::UNAUTHORIZED,
        "an operator's call"
    );
}

#[test]
fn names_a_task_after_its_job_so_scheduling_it_twice_makes_one() {
    use osiris_tasks::{Scheduled, TaskScheduler, adapters::memory::MemoryScheduler};
    let scheduler = MemoryScheduler::default();
    let task = |run_at| Task {
        name: names::refresh("ca_1", 3),
        run_at,
        payload: TaskPayload::ConsentExpire(osiris_tasks::ConsentTask {
            tenant_id: "org_1".into(),
            connected_account_id: "ca_1".into(),
        }),
    };
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let now = time::OffsetDateTime::now_utc();
    assert_eq!(
        runtime.block_on(scheduler.schedule(&task(now))).unwrap(),
        Scheduled::Created
    );
    assert_eq!(
        runtime
            .block_on(scheduler.schedule(&task(now + time::Duration::hours(1))))
            .unwrap(),
        Scheduled::AlreadyScheduled
    );
    assert_eq!(scheduler.pending().len(), 1);
}
