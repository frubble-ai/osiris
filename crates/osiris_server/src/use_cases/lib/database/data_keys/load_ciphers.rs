//! Reads the data keys of several tenants in one query and keeps them in memory, so a list page
//! decrypts its rows without a query per tenant. Tenants without a key yet are left for
//! `cipher_for` to make.

use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::Db;
use crate::use_cases::lib::database::Tenant;
use crate::use_cases::lib::database::errors::db;
use error_stack::ResultExt;
use osiris_domain::DomainError;
use std::sync::PoisonError;

impl Db {
    pub(crate) async fn load_ciphers(&self, tenants: &[Tenant]) -> UseCaseResult<()> {
        let missing: Vec<String> = {
            let keys = self.keys.lock().unwrap_or_else(PoisonError::into_inner);
            let mut missing: Vec<String> = tenants
                .iter()
                .map(Tenant::key)
                .filter(|key| !keys.contains_key(key))
                .collect();
            missing.sort();
            missing.dedup();
            missing
        };
        if missing.is_empty() {
            return Ok(());
        }
        let stored = sqlx::query!(
            "SELECT tenant, wrapped_key FROM data_key WHERE tenant = ANY($1)",
            &missing
        )
        .fetch_all(&self.pool)
        .await
        .map_err(db)?;
        for row in stored {
            let cipher = self
                .master
                .unwrap_data_key(&row.wrapped_key)
                .change_context(DomainError::Internal)?;
            self.keys
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .insert(row.tenant, cipher);
        }
        Ok(())
    }
}
