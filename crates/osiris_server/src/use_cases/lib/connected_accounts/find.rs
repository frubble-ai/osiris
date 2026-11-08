//! One connection, decrypted with its user's data key.

use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::Db;
use crate::use_cases::lib::database::Tenant;
use crate::use_cases::lib::database::errors::db;
use crate::use_cases::lib::database::rows::AccountRow;
use osiris_domain::ConnectedAccount;
use osiris_domain::DomainError;

impl Db {
    pub async fn connected_account(&self, id: &str) -> UseCaseResult<ConnectedAccount> {
        let row = sqlx::query_as!(AccountRow, "SELECT id, auth_config_id, provider, user_id, alias, status, credential_kind, installation_id, callback_url, access_token, refresh_token, access_token_expires_at, refresh_token_issued_at, scope, last_refresh_error, refresh_failures, token_generation, refresh_task, refresh_task_at, created_at, updated_at FROM connected_account WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
            .map_err(db)?
            .ok_or(DomainError::NotFound("connected account"))?;
        self.decrypt_account(row).await
    }

    /// A row, decrypted with its user's data key.
    pub(crate) async fn decrypt_account(&self, row: AccountRow) -> UseCaseResult<ConnectedAccount> {
        let cipher = self.cipher_for(&Tenant::user(&row.user_id)).await?;
        row.decrypt(&cipher)
    }
}
