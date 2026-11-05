//! Deletes a connection and returns the name of its pending refresh task, so the caller can
//! cancel it.

use crate::{
    Store,
    errors::{StorageError, StorageResult, db},
};

impl Store {
    pub async fn delete_connected_account(&self, id: &str) -> StorageResult<Option<String>> {
        let deleted = sqlx::query_scalar!(
            "DELETE FROM connected_account WHERE id = $1 RETURNING refresh_task",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(db)?
        .ok_or(StorageError::NotFound("connected account"))?;
        Ok(deleted)
    }
}
