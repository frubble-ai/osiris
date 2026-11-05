//! The cipher of one tenant's data key: from memory, or read (and made on first use).

use crate::{
    Store, Tenant,
    errors::{StorageError, StorageResult, db},
};
use error_stack::ResultExt;
use osiris_common::crypto::Cipher;
use std::sync::PoisonError;

impl Store {
    /// The cipher of `tenant`'s data key.
    pub async fn cipher_for(&self, tenant: &Tenant) -> StorageResult<Cipher> {
        let key = tenant.key();
        if let Some(cipher) = self
            .keys
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .get(&key)
        {
            return Ok(cipher.clone());
        }
        let found = sqlx::query_scalar!("SELECT wrapped_key FROM data_key WHERE tenant = $1", key)
            .fetch_optional(&self.pool)
            .await
            .map_err(db)?;
        let stored = if let Some(stored) = found {
            stored
        } else {
            let (_, wrapped) = self
                .master
                .new_data_key()
                .change_context(StorageError::Encrypt)?;
            // A concurrent first use may have made one: the update changes nothing, and returns the
            // row that was stored first, whichever transaction stored it.
            sqlx::query_scalar!(
                "INSERT INTO data_key (tenant, wrapped_key) VALUES ($1, $2)
                 ON CONFLICT (tenant) DO UPDATE SET tenant = EXCLUDED.tenant
                 RETURNING wrapped_key",
                key,
                wrapped
            )
            .fetch_one(&self.pool)
            .await
            .map_err(db)?
        };
        let cipher = self
            .master
            .unwrap_data_key(&stored)
            .change_context(StorageError::Decrypt)?;
        self.keys
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(key, cipher.clone());
        Ok(cipher)
    }
}
