//! What a check a delivery asked for saw, written alone: the trigger's schedule belongs to its
//! scheduled checks, so this never touches it.

use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::{Db, errors::db};

impl Db {
    /// Writes `checkpoint` on an active trigger; answers whether one was written.
    pub async fn update_trigger_checkpoint(
        &self,
        id: &str,
        checkpoint: &serde_json::Value,
    ) -> UseCaseResult<bool> {
        let written = sqlx::query!(
            "UPDATE trigger SET checkpoint = $2, last_error = NULL, updated_at = now()
             WHERE id = $1 AND status = 'active'",
            id,
            checkpoint,
        )
        .execute(&self.pool)
        .await
        .map_err(db)?;
        Ok(written.rows_affected() > 0)
    }
}
