//! Register a provider's app.

use crate::core::{Core, CoreResult};
use osiris_common::{
    errors::ReportSwitchExt,
    secret::{ExposeSecret, Secret},
};
use osiris_domain::{AuthConfig, DomainError};
use osiris_storage::auth_configs::NewAuthConfig;

pub struct CreateAuthConfig {
    pub provider: String,
    pub client_id: String,
    pub client_secret: Secret,
    pub scopes: Vec<String>,
}

impl Core {
    pub async fn create_auth_config(&self, new: CreateAuthConfig) -> CoreResult<AuthConfig> {
        if self.providers.get(&new.provider).is_none() {
            return Err(DomainError::Invalid(format!(
                "osiris does not implement the provider {:?}",
                new.provider
            ))
            .into());
        }
        if new.client_id.trim().is_empty() || new.client_secret.expose_secret().trim().is_empty() {
            return Err(
                DomainError::Invalid("client_id and client_secret are required".into()).into(),
            );
        }
        self.store
            .create_auth_config(NewAuthConfig {
                provider: new.provider,
                client_id: new.client_id,
                client_secret: new.client_secret,
                scopes: new.scopes,
            })
            .await
            .switch()
    }
}
