//! A row of `trigger`.

use super::decrypt;
use crate::use_cases::UseCaseResult;
use osiris_common::{crypto::Cipher, secret::secret};
use osiris_domain::{DomainError, Trigger, TriggerStatus};
use time::OffsetDateTime;

pub struct TriggerRow {
    pub id: String,
    pub connected_account_id: String,
    pub provider: String,
    pub kind: String,
    pub config: serde_json::Value,
    pub secret: Vec<u8>,
    pub status: String,
    pub subscription_id: Option<String>,
    pub checkpoint: Option<serde_json::Value>,
    pub last_error: Option<String>,
    pub poll_task: Option<String>,
    pub poll_task_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl TriggerRow {
    pub fn decrypt(self, cipher: &Cipher) -> UseCaseResult<Trigger> {
        Ok(Trigger {
            secret: secret(decrypt(cipher, &self.secret)?),
            status: TriggerStatus::parse(&self.status).ok_or(DomainError::Internal)?,
            id: self.id,
            connected_account_id: self.connected_account_id,
            provider: self.provider,
            kind: self.kind,
            config: self.config,
            subscription_id: self.subscription_id,
            checkpoint: self.checkpoint,
            last_error: self.last_error,
            poll_task: self.poll_task,
            poll_task_at: self.poll_task_at,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}
