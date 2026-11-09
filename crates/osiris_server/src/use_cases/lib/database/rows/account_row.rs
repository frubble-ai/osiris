//! A row of `connected_account`, without its consent's `state` and verifier.

use super::decrypt;
use crate::use_cases::UseCaseResult;
use osiris_common::{crypto::Cipher, secret::secret};
use osiris_domain::DomainError;
use osiris_domain::{ConnectedAccount, ConnectionStatus, CredentialKind, TokenSet};
use time::OffsetDateTime;

pub struct AccountRow {
    pub id: String,
    pub auth_config_id: String,
    pub provider: String,
    pub user_id: String,
    pub alias: Option<String>,
    pub status: String,
    pub credential_kind: String,
    pub installation_id: Option<String>,
    pub callback_url: String,
    pub access_token: Option<Vec<u8>>,
    pub refresh_token: Option<Vec<u8>>,
    pub access_token_expires_at: Option<OffsetDateTime>,
    pub refresh_token_issued_at: Option<OffsetDateTime>,
    pub scope: Option<String>,
    pub last_refresh_error: Option<String>,
    pub refresh_failures: i32,
    pub token_generation: i64,
    pub refresh_task: Option<String>,
    pub refresh_task_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl AccountRow {
    pub fn decrypt(self, cipher: &Cipher) -> UseCaseResult<ConnectedAccount> {
        let tokens = match &self.access_token {
            Some(access) => Some(TokenSet {
                access_token: secret(decrypt(cipher, access)?),
                refresh_token: self
                    .refresh_token
                    .as_deref()
                    .map(|value| decrypt(cipher, value).map(secret))
                    .transpose()?,
                access_token_expires_at: self.access_token_expires_at,
                refresh_token_issued_at: self.refresh_token_issued_at,
                scope: self.scope.clone(),
            }),
            None => None,
        };
        Ok(ConnectedAccount {
            status: ConnectionStatus::parse(&self.status).ok_or(DomainError::Internal)?,
            credential_kind: CredentialKind::parse(&self.credential_kind)
                .ok_or(DomainError::Internal)?,
            installation_id: self.installation_id,
            id: self.id,
            auth_config_id: self.auth_config_id,
            provider: self.provider,
            user_id: self.user_id,
            alias: self.alias,
            callback_url: self.callback_url,
            tokens,
            last_refresh_error: self.last_refresh_error,
            refresh_failures: self.refresh_failures,
            token_generation: self.token_generation,
            refresh_task: self.refresh_task,
            refresh_task_at: self.refresh_task_at,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}
