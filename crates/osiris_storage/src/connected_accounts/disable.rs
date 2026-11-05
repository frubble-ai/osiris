//! Disables a connection at connect: its consent finished, but either the provider sent no refresh
//! token or the first refresh could not be scheduled. Osiris keeps no tokens, so the user must
//! connect again.

use crate::{
    Store,
    errors::{StorageResult, db},
};

impl Store {
    pub async fn disable_consent(&self, id: &str, reason: &str) -> StorageResult<()> {
        sqlx::query!(
            "UPDATE connected_account SET status = 'disabled', last_refresh_error = $2, updated_at = now()
             WHERE id = $1 AND status = 'pending'",
            id,
            reason
        )
        .execute(&self.pool)
        .await
        .map_err(db)?;
        Ok(())
    }
}
