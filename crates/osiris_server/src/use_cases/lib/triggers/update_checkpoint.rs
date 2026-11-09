//! What a check a delivery asked for saw, written alone: the trigger's schedule belongs to its
//! scheduled checks, so this never touches it. It is written only over the checkpoint the check
//! started from, so a newer one another check wrote meanwhile is kept.

use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::{Db, errors::db};

impl Db {
    /// Writes `checkpoint` on an active trigger whose stored one is still `read_checkpoint`;
    /// answers whether one was written.
    pub async fn update_trigger_checkpoint(
        &self,
        id: &str,
        read_checkpoint: Option<&serde_json::Value>,
        checkpoint: &serde_json::Value,
    ) -> UseCaseResult<bool> {
        let written = sqlx::query!(
            "UPDATE trigger SET checkpoint = $2, last_error = NULL, updated_at = now()
             WHERE id = $1 AND status = 'active' AND checkpoint IS NOT DISTINCT FROM $3",
            id,
            checkpoint,
            read_checkpoint,
        )
        .execute(&self.pool)
        .await
        .map_err(db)?;
        Ok(written.rows_affected() > 0)
    }
}
