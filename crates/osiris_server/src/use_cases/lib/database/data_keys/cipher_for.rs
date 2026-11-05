//! The cipher of one tenant's data key: from memory, or read (and made on first use).

use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::Db;
use crate::use_cases::lib::database::Tenant;
use crate::use_cases::lib::database::errors::db;
use error_stack::ResultExt;
use osiris_common::crypto::Cipher;
use osiris_domain::DomainError;
use std::sync::PoisonError;

impl Db {
    /// The cipher of `tenant`'s data key.
    pub async fn cipher_for(&self, tenant: &Tenant) -> UseCaseResult<Cipher> {
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
                .change_context(DomainError::Internal)?;
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
            .change_context(DomainError::Internal)?;
        self.keys
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(key, cipher.clone());
        Ok(cipher)
    }
}
