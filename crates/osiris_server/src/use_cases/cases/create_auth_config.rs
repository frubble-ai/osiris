//! Registers a provider's app: its OAuth client, the scopes it asks for, and the provider's own
//! settings for it, which the provider validates.

use crate::use_cases::lib::auth_configs::{NewAuthConfig, validated_settings::validated_settings};
use crate::use_cases::{UseCaseResult, UseCases};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_domain::{AuthConfig, DomainError};

pub struct CreateAuthConfigArgs {
    pub provider: String,
    pub client_id: String,
    pub client_secret: Secret,
    pub scopes: Vec<String>,
    /// The provider's own settings, an object.
    pub settings: serde_json::Value,
    /// The provider's own secret settings, as JSON text.
    pub secret_settings: Option<Secret>,
}

impl UseCases {
    pub async fn create_auth_config(
        &self,
        args: CreateAuthConfigArgs,
    ) -> UseCaseResult<AuthConfig> {
        self.run("create_auth_config", async {
            let ctx = self.context();
            let Some(provider) = ctx.providers.get(&args.provider) else {
                return Err(DomainError::Invalid(format!(
                    "osiris does not implement the provider {:?}",
                    args.provider
                ))
                .into());
            };
            validated_settings(provider, &args.settings, args.secret_settings.as_ref())?;
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
                    settings: args.settings,
                    secret_settings: args.secret_settings,
                })
                .await
        })
        .await
    }
}
