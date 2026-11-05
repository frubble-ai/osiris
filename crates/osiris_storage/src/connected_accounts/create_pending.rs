//! Open a consent: a pending connection holding the OAuth `state` and the PKCE verifier, its
//! provider copied from its auth config.

use super::NewPending;
use crate::{
    Store, Tenant,
    errors::{StorageError, StorageResult, db},
    rows::{AccountRow, encrypt},
};
use osiris_common::{ids, secret::ExposeSecret};
use osiris_domain::ConnectedAccount;

impl Store {
    pub async fn create_pending(&self, new: NewPending) -> StorageResult<ConnectedAccount> {
        let cipher = self.cipher_for(&Tenant::user(&new.user_id)).await?;
        let verifier = new
            .pkce_verifier
            .as_ref()
            .map(|verifier| encrypt(&cipher, verifier.expose_secret()))
            .transpose()?;
        sqlx::query_as!(
            AccountRow,
            "INSERT INTO connected_account (id, auth_config_id, provider, user_id, alias, callback_url,
                 oauth_state, oauth_state_expires_at, pkce_verifier)
             SELECT $1, id, provider, $3, $4, $5, $6, $7, $8 FROM auth_config WHERE id = $2
             RETURNING id, auth_config_id, provider, user_id, alias, status, callback_url, access_token, refresh_token, access_token_expires_at, refresh_token_issued_at, scope, last_refresh_error, refresh_failures, token_generation, refresh_task, refresh_task_at, created_at, updated_at",
            ids::id("ca"),
            new.auth_config_id,
            new.user_id,
            new.alias,
            new.callback_url,
            new.oauth_state,
            new.oauth_state_expires_at,
            verifier,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(db)?
        .ok_or(StorageError::NotFound("auth config"))?
        .decrypt(&cipher)
    }
}
