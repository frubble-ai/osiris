//! A trigger's state after a check or a delivery, written whole.

use super::TriggerState;
use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::{Db, Tenant, errors::db, rows::TriggerRow};
use osiris_domain::{DomainError, Trigger};

impl Db {
    pub async fn update_trigger_state(
        &self,
        id: &str,
        state: TriggerState,
    ) -> UseCaseResult<Trigger> {
        let cipher = self.cipher_for(&Tenant::Osiris).await?;
        sqlx::query_as!(
            TriggerRow,
            "UPDATE trigger SET status = $2, checkpoint = $3, last_error = $4, poll_task = $5,
                 poll_task_at = $6, updated_at = now()
             WHERE id = $1 RETURNING id, connected_account_id, provider, kind, config, secret, status, subscription_id, checkpoint, last_error, poll_task, poll_task_at, created_at, updated_at",
            id,
            state.status.as_str(),
            state.checkpoint,
            state.last_error,
            state.poll_task,
            state.poll_task_at,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(db)?
        .ok_or(DomainError::NotFound("trigger"))?
        .decrypt(&cipher)
    }
}
