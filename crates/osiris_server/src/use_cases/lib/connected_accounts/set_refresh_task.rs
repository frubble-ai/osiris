//! Records a different refresh task for an active connection, for the same tokens (used when an
//! operator asks for an immediate refresh). Returns false when the connection is no longer active
//! or its tokens have changed.

use super::ScheduledRefresh;
use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::Db;
use crate::use_cases::lib::database::errors::db;

impl Db {
    pub async fn set_refresh_task(
        &self,
        id: &str,
        generation: i64,
        refresh: &ScheduledRefresh,
    ) -> UseCaseResult<bool> {
        let set = sqlx::query!(
            "UPDATE connected_account SET refresh_task = $3, refresh_task_at = $4, updated_at = now()
             WHERE id = $1 AND token_generation = $2 AND status = 'active'",
            id,
            generation,
            refresh.name,
            refresh.at,
        )
        .execute(&self.pool)
        .await
        .map_err(db)?
        .rows_affected();
        Ok(set == 1)
    }
}
