//! A trigger's state after a scheduled check, or after a check the provider refused, written
//! only while the trigger is still waiting for the check that read it. Its checkpoint is written
//! only if no other check moved it on meanwhile, so a newer one is kept.

use super::TriggerState;
use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::{Db, Tenant, errors::db, rows::TriggerRow};
use osiris_domain::Trigger;

impl Db {
    /// Writes `state` unless the trigger's pending check is no longer `expected_poll_task` (a
    /// concurrent check moved it on, or the trigger is gone); its checkpoint only if the stored
    /// one is still `read_checkpoint`, the one the check started from. Answers the trigger
    /// written, or none when nothing was.
    pub async fn update_trigger_state(
        &self,
        id: &str,
        expected_poll_task: Option<&str>,
        read_checkpoint: Option<&serde_json::Value>,
        state: TriggerState,
    ) -> UseCaseResult<Option<Trigger>> {
        let cipher = self.cipher_for(&Tenant::Osiris).await?;
        let written = sqlx::query_as!(
            TriggerRow,
            "UPDATE trigger SET status = $2,
                 checkpoint = CASE WHEN checkpoint IS NOT DISTINCT FROM $8 THEN $3 ELSE checkpoint END,
                 last_error = $4, poll_task = $5,
                 poll_task_at = $6, updated_at = now()
             WHERE id = $1 AND poll_task IS NOT DISTINCT FROM $7
             RETURNING id, connected_account_id, provider, kind, config, secret, status, subscription_id, checkpoint, last_error, poll_task, poll_task_at, created_at, updated_at",
            id,
            state.status.as_str(),
            state.checkpoint,
            state.last_error,
            state.poll_task,
            state.poll_task_at,
            expected_poll_task,
            read_checkpoint,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(db)?;
        written.map(|row| row.decrypt(&cipher)).transpose()
    }
}
