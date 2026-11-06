//! Registers a provider's app: its OAuth client and the scopes it asks for.

use crate::use_cases::lib::auth_configs::NewAuthConfig;
use crate::use_cases::{UseCaseResult, UseCases};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_domain::{AuthConfig, DomainError};

pub struct CreateAuthConfigArgs {
    pub provider: String,
    pub client_id: String,
    pub client_secret: Secret,
    pub scopes: Vec<String>,
}

impl UseCases {
    pub async fn create_auth_config(
        &self,
        args: CreateAuthConfigArgs,
    ) -> UseCaseResult<AuthConfig> {
        self.run("create_auth_config", async {
            let ctx = self.context();
            if ctx.providers.get(&args.provider).is_none() {
                return Err(DomainError::Invalid(format!(
                    "osiris does not implement the provider {:?}",
                    args.provider
                ))
                .into());
            }
            if args.client_id.trim().is_empty()
                || args.client_secret.expose_secret().trim().is_empty()
            {
                return Err(DomainError::Invalid(
                    "client_id and client_secret are required".into(),
                )
                .into());
            }
            ctx.db
                .create_auth_config(NewAuthConfig {
                    provider: args.provider,
                    client_id: args.client_id,
                    client_secret: args.client_secret,
                    scopes: args.scopes,
                })
                .await
        })
        .await
    }
}
