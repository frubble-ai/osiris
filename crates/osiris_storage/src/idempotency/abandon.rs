//! The request failed before answering: free the key for a retry.

use crate::{
    Store,
    errors::{StorageResult, db},
};

impl Store {
    pub async fn abandon_idempotent(&self, key: &str) -> StorageResult<()> {
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
