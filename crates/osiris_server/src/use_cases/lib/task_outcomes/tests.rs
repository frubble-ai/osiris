//! `task_outcome`: one row per task run, with old rows deleted as new ones are recorded.

use super::Recorded;
use crate::testing::db_fixtures::db;
use osiris_domain::TaskOutcome;
use sqlx::PgPool;
use time::Duration;

fn recorded(name: &str, outcome: TaskOutcome) -> Recorded<'_> {
    Recorded {
        task_name: name,
        kind: "connection.refresh",
        connected_account_id: "ca_1",
        outcome,
        detail: None,
    }
}

#[sqlx::test(migrator = "crate::db::MIGRATOR")]
async fn records_each_run_and_deletes_rows_past_their_retention(pool: PgPool) {
    let db = db(pool.clone());
    db.record_task_outcome(recorded("t1", TaskOutcome::Failed), Duration::days(1), 100)
        .await
        .unwrap();
    sqlx::query!("UPDATE task_outcome SET recorded_at = now() - interval '2 days'")
        .execute(&pool)
        .await
        .unwrap();
    db.record_task_outcome(
        recorded("t1", TaskOutcome::Refreshed),
        Duration::days(1),
        100,
    )
    .await
    .unwrap();
    let outcomes = sqlx::query_scalar!("SELECT outcome FROM task_outcome")
        .fetch_all(&pool)
        .await
        .unwrap();
    assert_eq!(outcomes, ["refreshed"]);
}
