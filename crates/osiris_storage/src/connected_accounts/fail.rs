//! The consent was refused or its code could not be exchanged. No event: the browser is sent back
//! to the client with `status=failed`, and the client asked for the connection itself.

use crate::{
    Store,
    errors::{StorageResult, db},
};

impl Store {
    pub async fn fail(&self, id: &str, reason: &str) -> StorageResult<()> {
        sqlx::query!("UPDATE connected_account SET status = 'failed', last_refresh_error = $2, updated_at = now() WHERE id = $1", id, reason)
            .execute(&self.pool)
            .await
            .map_err(db)?;
        Ok(())
    }
}
