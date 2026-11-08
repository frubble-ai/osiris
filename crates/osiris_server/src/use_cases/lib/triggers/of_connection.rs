//! Every trigger of one connection, for ending their subscriptions before it is deleted.

use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::{Db, Tenant, errors::db, rows::TriggerRow};
use osiris_domain::Trigger;

impl Db {
    pub async fn connection_triggers(
        &self,
        connected_account_id: &str,
    ) -> UseCaseResult<Vec<Trigger>> {
        let cipher = self.cipher_for(&Tenant::Osiris).await?;
        sqlx::query_as!(
            TriggerRow,
            "SELECT id, connected_account_id, provider, kind, config, secret, status, subscription_id, checkpoint, last_error, poll_task, poll_task_at, created_at, updated_at
             FROM trigger WHERE connected_account_id = $1 ORDER BY created_at, id",
            connected_account_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(db)?
        .into_iter()
        .map(|row| row.decrypt(&cipher))
        .collect()
    }
}
