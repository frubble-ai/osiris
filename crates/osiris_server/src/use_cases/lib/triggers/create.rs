//! Store a trigger, its provider copied from its connection and its secret encrypted with osiris's
//! own data key.

use super::NewTrigger;
use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::{
    Db, Tenant,
    errors::db,
    rows::{TriggerRow, encrypt},
};
use osiris_common::secret::ExposeSecret;
use osiris_domain::{DomainError, Trigger};

impl Db {
    pub async fn create_trigger(&self, new: NewTrigger) -> UseCaseResult<Trigger> {
        let cipher = self.cipher_for(&Tenant::Osiris).await?;
        let secret = encrypt(&cipher, new.secret.expose_secret())?;
        sqlx::query_as!(
            TriggerRow,
            "INSERT INTO trigger (id, connected_account_id, provider, kind, config, secret, status,
                 subscription_id, checkpoint, last_error, poll_task, poll_task_at)
             SELECT $1, id, provider, $3, $4, $5, $6, $7, $8, $9, $10, $11
             FROM connected_account WHERE id = $2
             RETURNING id, connected_account_id, provider, kind, config, secret, status, subscription_id, checkpoint, last_error, poll_task, poll_task_at, created_at, updated_at",
            new.id,
            new.connected_account_id,
            new.kind,
            new.config,
            secret,
            new.status.as_str(),
            new.subscription_id,
            new.checkpoint,
            new.last_error,
            new.poll_task,
            new.poll_task_at,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(db)?
        .ok_or(DomainError::NotFound("connected account"))?
        .decrypt(&cipher)
    }
}
