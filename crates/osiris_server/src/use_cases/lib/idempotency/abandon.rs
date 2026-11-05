//! The request failed before answering: free the key for a retry.

use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::Db;
use crate::use_cases::lib::database::errors::db;
impl Db {
    pub async fn abandon_idempotent(&self, key: &str) -> UseCaseResult<()> {
        sqlx::query!(
            "DELETE FROM idempotency_key WHERE key = $1 AND status IS NULL",
            key
        )
        .execute(&self.pool)
        .await
        .map_err(db)?;
        Ok(())
    }
}
