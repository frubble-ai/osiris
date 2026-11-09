//! One provider's app.

use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::Db;
use crate::use_cases::lib::database::Tenant;
use crate::use_cases::lib::database::errors::db;
use crate::use_cases::lib::database::rows::AuthConfigRow;
use osiris_domain::AuthConfig;
use osiris_domain::DomainError;

impl Db {
    pub async fn auth_config(&self, id: &str) -> UseCaseResult<AuthConfig> {
        let cipher = self.cipher_for(&Tenant::Osiris).await?;
        sqlx::query_as!(AuthConfigRow, "SELECT id, provider, client_id, client_secret, scopes, settings, secret_settings, created_at, updated_at FROM auth_config WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
            .map_err(db)?
            .ok_or(DomainError::NotFound("auth config"))?
            .decrypt(&cipher)
    }
}
