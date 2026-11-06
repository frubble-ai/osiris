//! Records how a task run ended. Each time, it also deletes a few rows older than the retention
//! period, so the table does not need a separate clean-up job.

use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::Db;
use crate::use_cases::lib::database::errors::db;
use osiris_domain::TaskOutcome;

/// How one task run ended.
pub struct Recorded<'a> {
    pub task_name: &'a str,
    pub kind: &'a str,
    pub connected_account_id: &'a str,
    pub outcome: TaskOutcome,
    pub detail: Option<&'a str>,
}

impl Db {
    pub async fn record_task_outcome(
        &self,
        recorded: Recorded<'_>,
        retention: time::Duration,
        prune_batch: i64,
    ) -> UseCaseResult<()> {
        let mut tx = self.pool.begin().await.map_err(db)?;
        sqlx::query!(
            "DELETE FROM task_outcome WHERE id IN (
                 SELECT id FROM task_outcome WHERE recorded_at < now() - make_interval(secs => $1) LIMIT $2)",
            retention.as_seconds_f64(),
            prune_batch,
        )
        .execute(&mut *tx)
        .await
        .map_err(db)?;
        sqlx::query!(
            "INSERT INTO task_outcome (task_name, kind, connected_account_id, outcome, detail) VALUES ($1, $2, $3, $4, $5)",
            recorded.task_name,
            recorded.kind,
            recorded.connected_account_id,
            recorded.outcome.as_str(),
            recorded.detail,
        )
        .execute(&mut *tx)
        .await
        .map_err(db)?;
        tx.commit().await.map_err(db)?;
        Ok(())
    }
}
