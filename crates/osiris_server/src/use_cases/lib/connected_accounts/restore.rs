//! Makes a disabled connection active again once its refresh task is scheduled. Only applies while
//! the connection is still disabled, on generation `generation`, with no task; returns false
//! otherwise (someone else restored or changed it first).

use super::ScheduledRefresh;
use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::Db;
use crate::use_cases::lib::database::errors::db;

impl Db {
    pub async fn restore(
        &self,
        id: &str,
        generation: i64,
        refresh: &ScheduledRefresh,
    ) -> UseCaseResult<bool> {
        let restored = sqlx::query!(
            "UPDATE connected_account SET status = 'active', refresh_task = $3, refresh_task_at = $4,
                 last_refresh_error = NULL, updated_at = now()
             WHERE id = $1 AND token_generation = $2 AND status = 'disabled' AND refresh_task IS NULL
               AND access_token IS NOT NULL",
            id,
            generation,
            refresh.name,
            refresh.at,
        )
        .execute(&self.pool)
        .await
        .map_err(db)?
        .rows_affected();
        Ok(restored == 1)
    }
}
