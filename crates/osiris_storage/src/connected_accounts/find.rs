//! One connection, decrypted with its user's data key.

use crate::{
    Store, Tenant,
    errors::{StorageError, StorageResult, db},
    rows::AccountRow,
};
use osiris_domain::ConnectedAccount;

impl Store {
    pub async fn connected_account(&self, id: &str) -> StorageResult<ConnectedAccount> {
        let row = sqlx::query_as!(AccountRow, "SELECT id, auth_config_id, provider, user_id, alias, status, callback_url, access_token, refresh_token, access_token_expires_at, refresh_token_issued_at, scope, last_refresh_error, refresh_failures, token_generation, refresh_task, refresh_task_at, created_at, updated_at FROM connected_account WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
            .map_err(db)?
            .ok_or(StorageError::NotFound("connected account"))?;
        self.decrypt_account(row).await
    }

    /// A row, decrypted with its user's data key.
    pub(crate) async fn decrypt_account(&self, row: AccountRow) -> StorageResult<ConnectedAccount> {
        let cipher = self.cipher_for(&Tenant::user(&row.user_id)).await?;
        row.decrypt(&cipher)
    }
}
