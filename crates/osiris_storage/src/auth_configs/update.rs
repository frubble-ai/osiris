//! Change an app's client, secret or scopes.

use super::AuthConfigChanges;
use crate::{
    Store, Tenant,
    errors::{StorageError, StorageResult, db},
    rows::{AuthConfigRow, encrypt},
};
use osiris_common::secret::ExposeSecret;
use osiris_domain::AuthConfig;

impl Store {
    pub async fn update_auth_config(
        &self,
        id: &str,
        changes: AuthConfigChanges,
    ) -> StorageResult<AuthConfig> {
        let cipher = self.cipher_for(&Tenant::Osiris).await?;
        let secret = changes
            .client_secret
            .map(|secret| encrypt(&cipher, secret.expose_secret()))
            .transpose()?;
        sqlx::query_as!(
            AuthConfigRow,
            "UPDATE auth_config SET client_id = COALESCE($2, client_id), client_secret = COALESCE($3, client_secret),
                 scopes = COALESCE($4, scopes), updated_at = now()
             WHERE id = $1 RETURNING id, provider, client_id, client_secret, scopes, created_at, updated_at",
            id,
            changes.client_id,
            secret,
            changes.scopes.as_deref(),
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(db)?
        .ok_or(StorageError::NotFound("auth config"))?
        .decrypt(&cipher)
    }
}
