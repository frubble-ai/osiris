//! The providers' apps, in batches (cursor pagination): `after` gives the next batch, `before`
//! the previous one.

use crate::{
    Cursor, Page, Store, Tenant,
    errors::{StorageResult, db},
    rows::AuthConfigRow,
};
use osiris_domain::AuthConfig;

impl Store {
    pub async fn list_auth_configs(&self, cursor: &Cursor) -> StorageResult<Page<AuthConfig>> {
        let cipher = self.cipher_for(&Tenant::Osiris).await?;
        // One static query per direction, so `(created_at, id)` serves the order.
        let found = match cursor.before.as_deref() {
            None => sqlx::query_as!(
                AuthConfigRow,
                "SELECT id, provider, client_id, client_secret, scopes, created_at, updated_at FROM auth_config
             WHERE ($1::text IS NULL OR (created_at, id) > (SELECT created_at, id FROM auth_config WHERE id = $1))
             ORDER BY created_at ASC, id ASC
             LIMIT $2",
                cursor.after.as_deref(),
                cursor.limit() + 1,
            )
            .fetch_all(&self.pool)
            .await,
            Some(before) => sqlx::query_as!(
                AuthConfigRow,
                "SELECT id, provider, client_id, client_secret, scopes, created_at, updated_at FROM auth_config
             WHERE (created_at, id) < (SELECT created_at, id FROM auth_config WHERE id = $1)
             ORDER BY created_at DESC, id DESC
             LIMIT $2",
                before,
                cursor.limit() + 1,
            )
            .fetch_all(&self.pool)
            .await,
        }
        .map_err(db)?
        .into_iter()
        .map(|row| row.decrypt(&cipher))
        .collect::<StorageResult<Vec<_>>>()?;
        Ok(Page::from_overfetch(found, cursor, |config| {
            config.id.clone()
        }))
    }
}
