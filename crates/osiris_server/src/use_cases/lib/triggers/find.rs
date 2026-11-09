//! One trigger.

use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::{Db, Tenant, errors::db, rows::TriggerRow};
use osiris_domain::{DomainError, Trigger};

impl Db {
    pub async fn trigger(&self, id: &str) -> UseCaseResult<Trigger> {
        let cipher = self.cipher_for(&Tenant::Osiris).await?;
        sqlx::query_as!(TriggerRow, "SELECT id, connected_account_id, provider, kind, config, secret, status, subscription_id, checkpoint, last_error, poll_task, poll_task_at, created_at, updated_at FROM trigger WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
            .map_err(db)?
            .ok_or(DomainError::NotFound("trigger"))?
            .decrypt(&cipher)
    }
}
