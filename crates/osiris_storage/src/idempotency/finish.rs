//! Keep the first answer, to replay.

use crate::{
    Store,
    errors::{StorageResult, db},
};
use serde_json::Value;

impl Store {
    pub async fn finish_idempotent(
        &self,
        key: &str,
        status: u16,
        body: &Value,
    ) -> StorageResult<()> {
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
