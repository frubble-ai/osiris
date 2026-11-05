//! One provider's app.

use crate::{
    Store, Tenant,
    errors::{StorageError, StorageResult, db},
    rows::AuthConfigRow,
};
use osiris_domain::AuthConfig;

impl Store {
    pub async fn auth_config(&self, id: &str) -> StorageResult<AuthConfig> {
        let cipher = self.cipher_for(&Tenant::Osiris).await?;
        sqlx::query_as!(AuthConfigRow, "SELECT id, provider, client_id, client_secret, scopes, created_at, updated_at FROM auth_config WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
            .map_err(db)?
            .ok_or(StorageError::NotFound("auth config"))?
            .decrypt(&cipher)
    }
}
