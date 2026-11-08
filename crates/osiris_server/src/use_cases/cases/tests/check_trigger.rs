//! A trigger's scheduled check, against the fake provider: a config the provider can't take fails
//! the trigger, and an answer keeps it checking.

#![allow(
    clippy::unwrap_used,
    reason = "tests: a failure here is the test failing"
)]

use crate::testing::{db_fixtures::generate_trigger, setup_test_context};
use crate::use_cases::cases::check_trigger::CheckTriggerArgs;
use osiris_domain::{TaskOutcome, TriggerStatus};
use serde_json::json;
use sqlx::PgPool;

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn fails_a_trigger_whose_config_the_provider_cannot_take(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let connection = app.connected(28800).await;
    let db = &app.use_cases.context().db;
    let task = format!("trigger-poll-{connection}-1");
    let mut trigger = generate_trigger(&connection, "a-secret", Some(&task));
    trigger.config = json!({ "unreadable": true });
    let trigger = db.create_trigger(trigger).await.unwrap();

    let (outcome, _) = app
        .use_cases
        .check_trigger(CheckTriggerArgs {
            trigger_id: &trigger.id,
            task: Some((&task, 1)),
        })
        .await
        .unwrap();

    assert_eq!(outcome, TaskOutcome::Failed);
    let after = db.trigger(&trigger.id).await.unwrap();
    assert_eq!(after.status, TriggerStatus::Failed);
    assert!(after.poll_task.is_none());
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn keeps_a_trigger_checking_when_the_provider_answers(pool: PgPool) {
    let app = setup_test_context(pool).await;
    let connection = app.connected(28800).await;
    let db = &app.use_cases.context().db;
    let task = format!("trigger-poll-{connection}-1");
    let trigger = db
        .create_trigger(generate_trigger(&connection, "a-secret", Some(&task)))
        .await
        .unwrap();

    let (outcome, _) = app
        .use_cases
        .check_trigger(CheckTriggerArgs {
            trigger_id: &trigger.id,
            task: Some((&task, 1)),
        })
        .await
        .unwrap();

    assert_eq!(outcome, TaskOutcome::TriggerChecked);
    let after = db.trigger(&trigger.id).await.unwrap();
    assert_eq!(after.status, TriggerStatus::Active);
    assert!(after.poll_task.is_some_and(|next| next != task));
}
