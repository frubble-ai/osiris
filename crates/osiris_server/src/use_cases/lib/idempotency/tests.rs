//! `idempotency_key`: one run per key, replays, refusals, and expired keys read as unused and
//! deleted as new keys are written.

use crate::testing::db_fixtures::db;
use osiris_domain::Begun;
use serde_json::json;
use sqlx::PgPool;
use time::Duration;

const DAY: Duration = Duration::days(1);

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn replays_an_idempotent_answer_and_refuses_the_key_for_another_request(pool: PgPool) {
    let db = db(pool);
    assert!(matches!(
        db.begin_idempotent("key-1", "hash-a", DAY, 100)
            .await
            .unwrap(),
        Begun::New
    ));
    assert!(matches!(
        db.begin_idempotent("key-1", "hash-a", DAY, 100)
            .await
            .unwrap(),
        Begun::InFlight
    ));
    db.finish_idempotent("key-1", 201, &json!({ "id": "ca_1" }))
        .await
        .unwrap();
    match db
        .begin_idempotent("key-1", "hash-a", DAY, 100)
        .await
        .unwrap()
    {
        Begun::Replay { status, body } => {
            assert_eq!(status, 201);
            assert_eq!(body["id"], "ca_1");
        }
        _ => panic!("expected a replay"),
    }
    assert!(matches!(
        db.begin_idempotent("key-1", "hash-b", DAY, 100)
            .await
            .unwrap(),
        Begun::Mismatch
    ));
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn treats_an_expired_key_as_unused(pool: PgPool) {
    let db = db(pool.clone());
    db.begin_idempotent("key-1", "hash-a", DAY, 100)
        .await
        .unwrap();
    db.finish_idempotent("key-1", 201, &json!({ "id": "ca_1" }))
        .await
        .unwrap();
    sqlx::query!("UPDATE idempotency_key SET created_at = now() - interval '2 days'")
        .execute(&pool)
        .await
        .unwrap();
    // Another request with the expired key runs as new, not refused and not replayed.
    assert!(matches!(
        db.begin_idempotent("key-1", "hash-b", DAY, 100)
            .await
            .unwrap(),
        Begun::New
    ));
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn deletes_expired_keys_as_new_ones_are_written(pool: PgPool) {
    let db = db(pool.clone());
    db.begin_idempotent("old", "h", DAY, 100).await.unwrap();
    sqlx::query!("UPDATE idempotency_key SET created_at = now() - interval '2 days'")
        .execute(&pool)
        .await
        .unwrap();
    db.begin_idempotent("new", "h", DAY, 100).await.unwrap();
    let keys = sqlx::query_scalar!("SELECT key FROM idempotency_key ORDER BY key")
        .fetch_all(&pool)
        .await
        .unwrap();
    assert_eq!(keys, ["new"]);
}
