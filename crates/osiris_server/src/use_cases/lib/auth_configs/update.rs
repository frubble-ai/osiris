//! Change an app's client, secret or scopes.

use super::AuthConfigChanges;
use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::Db;
use crate::use_cases::lib::database::Tenant;
use crate::use_cases::lib::database::errors::db;
use crate::use_cases::lib::database::rows::AuthConfigRow;
use crate::use_cases::lib::database::rows::encrypt;
use osiris_common::secret::ExposeSecret;
use osiris_domain::AuthConfig;
use osiris_domain::DomainError;

impl Db {
    pub async fn update_auth_config(
        &self,
        id: &str,
        changes: AuthConfigChanges,
    ) -> UseCaseResult<AuthConfig> {
        let cipher = self.cipher_for(&Tenant::Osiris).await?;
        let secret = changes
            .client_secret
            .map(|secret| encrypt(&cipher, secret.expose_secret()))
            .transpose()?;
        sqlx::query_as!(
            AuthConfigRow,
            "UPDATE auth_config SET client_id = COALESCE($2, client_id), client_secret = COALESCE($3, client_secret),
                 scopes = COALESCE($4, scopes), updated_at = now()
             WHERE id = $1 RETURNING id, provider, client_id, client_secret, scopes, created_at, updated_at",
            id,
            changes.client_id,
            secret,
            changes.scopes.as_deref(),
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(db)?
        .ok_or(DomainError::NotFound("auth config"))?
        .decrypt(&cipher)
    }
}
