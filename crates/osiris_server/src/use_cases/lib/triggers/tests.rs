//! `trigger` against a real Postgres: its secret encrypted, its pages, its state written only for
//! the check the trigger waits for, and a connection's triggers.

use super::TriggerState;
use crate::testing::db_fixtures::{
    active, db, generate_secret, generate_state, generate_trigger, generate_user_id,
};
use crate::use_cases::lib::database::Db;
use osiris_common::secret::ExposeSecret;
use osiris_domain::{Cursor, DomainError, TriggerStatus};
use serde_json::json;
use sqlx::PgPool;

/// An active connection of the fake provider; answers its id.
async fn connection(db: &Db) -> String {
    active(db, &generate_user_id(), &generate_state(), Some(3600)).await
}

/// An active trigger on `connected_account_id` waiting for `poll_task`; answers its id.
async fn trigger(db: &Db, connected_account_id: &str, poll_task: Option<&str>) -> String {
    db.create_trigger(generate_trigger(
        connected_account_id,
        &generate_secret("trigger-secret"),
        poll_task,
    ))
    .await
    .unwrap()
    .id
}

/// A state written by a scheduled check that answered: still active, waiting for `poll_task`.
fn checked(poll_task: &str) -> TriggerState {
    TriggerState {
        status: TriggerStatus::Active,
        checkpoint: Some(json!({ "version": generate_secret("version") })),
        last_error: None,
        poll_task: Some(poll_task.into()),
        poll_task_at: None,
    }
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn stores_the_secret_encrypted_and_reads_it_back(pool: PgPool) {
    let db = db(pool.clone());
    let account = connection(&db).await;
    let trigger_secret = generate_secret("trigger-secret");

    let created = db
        .create_trigger(generate_trigger(&account, &trigger_secret, None))
        .await
        .unwrap();

    assert_eq!(created.secret.expose_secret(), trigger_secret);
    assert_eq!(created.provider, osiris_providers::mocks::FAKE_ID);
    let raw = sqlx::query_scalar!("SELECT secret FROM trigger WHERE id = $1", created.id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(!String::from_utf8_lossy(&raw).contains(&trigger_secret));
    // A fresh db has no key in memory: the read decrypts with the key it loads.
    let reader = crate::testing::db_fixtures::db(pool);
    assert_eq!(
        reader
            .trigger(&created.id)
            .await
            .unwrap()
            .secret
            .expose_secret(),
        trigger_secret
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn lists_triggers_a_page_at_a_time_forwards_and_back(pool: PgPool) {
    let db = db(pool);
    let account = connection(&db).await;
    for _ in 0..3 {
        trigger(&db, &account, None).await;
    }
    // Another connection's trigger is not in this connection's pages.
    let other = connection(&db).await;
    trigger(&db, &other, None).await;
    let page = |cursor: Cursor| {
        let (db, account) = (&db, &account);
        async move { db.list_triggers(Some(account), &cursor).await.unwrap() }
    };

    let first = page(Cursor {
        limit: 2,
        ..Default::default()
    })
    .await;
    assert_eq!(first.items.len(), 2);
    assert!(first.next.is_some());
    let second = page(Cursor {
        after: first.next.clone(),
        limit: 2,
        ..Default::default()
    })
    .await;
    assert_eq!(second.items.len(), 1);
    assert!(second.next.is_none());
    assert!(
        second
            .items
            .iter()
            .all(|found| found.connected_account_id == account)
    );
    let back = page(Cursor {
        before: second.previous.clone(),
        limit: 2,
        ..Default::default()
    })
    .await;
    let ids = |items: &[osiris_domain::Trigger]| {
        items
            .iter()
            .map(|found| found.id.clone())
            .collect::<Vec<_>>()
    };
    assert_eq!(ids(&back.items), ids(&first.items));

    let everyone = db
        .list_triggers(
            None,
            &Cursor {
                limit: 10,
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(everyone.items.len(), 4);
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn refuses_a_cursor_naming_a_deleted_trigger(pool: PgPool) {
    let db = db(pool);
    let account = connection(&db).await;
    let deleted = trigger(&db, &account, None).await;
    db.delete_trigger(&deleted).await.unwrap();

    for cursor in [
        Cursor {
            after: Some(deleted.clone()),
            limit: 2,
            ..Default::default()
        },
        Cursor {
            before: Some(deleted.clone()),
            limit: 2,
            ..Default::default()
        },
    ] {
        let failure = db.list_triggers(None, &cursor).await.unwrap_err();
        assert!(
            matches!(failure.current_context(), DomainError::Invalid(_)),
            "{failure:?}"
        );
    }
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn writes_a_state_only_while_the_trigger_waits_for_that_check(pool: PgPool) {
    let db = db(pool);
    let account = connection(&db).await;
    let id = trigger(&db, &account, Some("check-1")).await;

    // A check the trigger no longer waits for writes nothing.
    assert!(
        db.update_trigger_state(&id, Some("check-0"), checked("check-9"))
            .await
            .unwrap()
            .is_none()
    );
    assert!(
        db.update_trigger_state(&id, None, checked("check-9"))
            .await
            .unwrap()
            .is_none()
    );
    assert_eq!(
        db.trigger(&id).await.unwrap().poll_task.as_deref(),
        Some("check-1")
    );

    let state = checked("check-2");
    let checkpoint = state.checkpoint.clone();
    let written = db
        .update_trigger_state(&id, Some("check-1"), state)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(written.poll_task.as_deref(), Some("check-2"));
    assert_eq!(written.checkpoint, checkpoint);
    // The check that just ran is no longer the one the trigger waits for.
    assert!(
        db.update_trigger_state(&id, Some("check-1"), checked("check-3"))
            .await
            .unwrap()
            .is_none()
    );
    // A trigger that is gone writes nothing either.
    db.delete_trigger(&id).await.unwrap();
    assert!(
        db.update_trigger_state(&id, Some("check-2"), checked("check-3"))
            .await
            .unwrap()
            .is_none()
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn writes_a_checkpoint_only_on_an_active_trigger_and_leaves_its_schedule(pool: PgPool) {
    let db = db(pool);
    let account = connection(&db).await;
    let id = trigger(&db, &account, Some("check-1")).await;
    let before = db.trigger(&id).await.unwrap();

    let seen = json!({ "version": "a" });
    assert!(db.update_trigger_checkpoint(&id, &seen).await.unwrap());
    let after = db.trigger(&id).await.unwrap();
    assert_eq!(after.checkpoint, Some(seen));
    assert_eq!(after.poll_task, before.poll_task);
    assert_eq!(after.poll_task_at, before.poll_task_at);

    let failed = TriggerState {
        status: TriggerStatus::Failed,
        checkpoint: after.checkpoint.clone(),
        last_error: Some("refused".into()),
        poll_task: None,
        poll_task_at: None,
    };
    db.update_trigger_state(&id, Some("check-1"), failed)
        .await
        .unwrap()
        .unwrap();
    assert!(
        !db.update_trigger_checkpoint(&id, &json!({ "version": "b" }))
            .await
            .unwrap()
    );
    let unchanged = db.trigger(&id).await.unwrap();
    assert_eq!(unchanged.status, TriggerStatus::Failed);
    assert_eq!(unchanged.checkpoint, after.checkpoint);
    assert_eq!(unchanged.last_error.as_deref(), Some("refused"));
    assert!(
        !db.update_trigger_checkpoint("tr_missing", &json!({}))
            .await
            .unwrap()
    );
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn answers_the_triggers_of_one_connection(pool: PgPool) {
    let db = db(pool);
    let account = connection(&db).await;
    let other = connection(&db).await;
    let first = trigger(&db, &account, None).await;
    let second = trigger(&db, &account, Some("check-1")).await;
    trigger(&db, &other, None).await;

    let found = db.connection_triggers(&account).await.unwrap();

    assert_eq!(
        found
            .iter()
            .map(|found| found.id.as_str())
            .collect::<Vec<_>>(),
        [first.as_str(), second.as_str()]
    );
    assert!(
        db.connection_triggers("ca_missing")
            .await
            .unwrap()
            .is_empty()
    );
}
