//! Store a connection that acts as an app's installation: active from the start, holding no
//! tokens (they are minted when an action runs), its provider copied from its auth config.

use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::Db;
use crate::use_cases::lib::database::Tenant;
use crate::use_cases::lib::database::errors::db;
use crate::use_cases::lib::database::rows::AccountRow;
use osiris_common::ids;
use osiris_domain::ConnectedAccount;
use osiris_domain::DomainError;

/// What an installation connection is made of.
pub struct NewInstallation {
    pub auth_config_id: String,
    pub user_id: String,
    pub alias: Option<String>,
    /// The provider's id of the installation it acts as.
    pub installation_id: String,
}

impl Db {
    pub async fn create_installation(
        &self,
        new: NewInstallation,
    ) -> UseCaseResult<ConnectedAccount> {
        let cipher = self.cipher_for(&Tenant::user(&new.user_id)).await?;
        sqlx::query_as!(
            AccountRow,
            "INSERT INTO connected_account (id, auth_config_id, provider, user_id, alias, callback_url,
                 status, credential_kind, installation_id)
             SELECT $1, id, provider, $3, $4, '', 'active', 'app_installation', $5 FROM auth_config WHERE id = $2
             RETURNING id, auth_config_id, provider, user_id, alias, status, credential_kind, installation_id, callback_url, access_token, refresh_token, access_token_expires_at, refresh_token_issued_at, scope, last_refresh_error, refresh_failures, token_generation, refresh_task, refresh_task_at, created_at, updated_at",
            ids::id("ca"),
            new.auth_config_id,
            new.user_id,
            new.alias,
            new.installation_id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(db)?
        .ok_or(DomainError::NotFound("auth config"))?
        .decrypt(&cipher)
    }
}
