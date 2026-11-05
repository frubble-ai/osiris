//! Register a provider's app.

use super::NewAuthConfig;
use crate::{
    Store, Tenant,
    errors::{StorageResult, db},
    rows::{AuthConfigRow, encrypt},
};
use osiris_common::{ids, secret::ExposeSecret};
use osiris_domain::AuthConfig;

impl Store {
    pub async fn create_auth_config(&self, new: NewAuthConfig) -> StorageResult<AuthConfig> {
        let cipher = self.cipher_for(&Tenant::Osiris).await?;
        let secret = encrypt(&cipher, new.client_secret.expose_secret())?;
        sqlx::query_as!(
            AuthConfigRow,
            "INSERT INTO auth_config (id, provider, client_id, client_secret, scopes) VALUES ($1, $2, $3, $4, $5)
             RETURNING id, provider, client_id, client_secret, scopes, created_at, updated_at",
            ids::id("ac"),
            new.provider,
            new.client_id,
            secret,
            &new.scopes,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(db)?
        .decrypt(&cipher)
    }
}
