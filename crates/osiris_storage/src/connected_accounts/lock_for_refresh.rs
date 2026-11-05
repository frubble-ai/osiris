//! Lock a connection for a refresh: concurrent refreshes of it wait here, then see the tokens the
//! first one stored.

use super::RefreshLock;
use crate::{
    Store, Tenant,
    errors::{StorageError, StorageResult, db},
    rows::AccountRow,
};

impl Store {
    pub async fn lock_for_refresh(&self, id: &str) -> StorageResult<RefreshLock> {
        let mut tx = self.pool.begin().await.map_err(db)?;
        // A refresh waiting on another one's lock gives up after `lock_timeout`, so waiters can't
        // hold the pool's connections for the length of a provider call.
        let milliseconds = u64::try_from(self.lock_timeout.as_millis()).unwrap_or(u64::MAX);
        sqlx::query_scalar!(
            "SELECT set_config('lock_timeout', $1, true)",
            format!("{milliseconds}ms")
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(db)?;
        let row = sqlx::query_as!(AccountRow, "SELECT id, auth_config_id, provider, user_id, alias, status, callback_url, access_token, refresh_token, access_token_expires_at, refresh_token_issued_at, scope, last_refresh_error, refresh_failures, token_generation, refresh_task, refresh_task_at, created_at, updated_at FROM connected_account WHERE id = $1 FOR UPDATE", id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(db)?
            .ok_or(StorageError::NotFound("connected account"))?;
        let cipher = self.cipher_for(&Tenant::user(&row.user_id)).await?;
        let account = row.decrypt(&cipher)?;
        Ok(RefreshLock::new(tx, self.clone(), cipher, account))
    }
}
