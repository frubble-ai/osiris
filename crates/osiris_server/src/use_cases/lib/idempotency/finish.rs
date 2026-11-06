//! Keep the first answer, to replay.

use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::Db;
use crate::use_cases::lib::database::errors::db;
use serde_json::Value;

impl Db {
    pub async fn finish_idempotent(
        &self,
        key: &str,
        status: u16,
        body: &Value,
    ) -> UseCaseResult<()> {
        sqlx::query!(
            "UPDATE idempotency_key SET status = $2, body = $3 WHERE key = $1",
            key,
            i32::from(status),
            body
        )
        .execute(&self.pool)
        .await
        .map_err(db)?;
        Ok(())
    }
}
