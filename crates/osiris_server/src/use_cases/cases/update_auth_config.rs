//! Changes an app's client, secret, scopes or settings. Connections made before keep the scopes
//! they got; installation tokens minted with the app's old credentials are dropped.

use crate::use_cases::lib::auth_configs::{
    AuthConfigChanges, validated_settings::validated_settings,
};
use crate::use_cases::{UseCaseResult, UseCases};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_domain::{AuthConfig, DomainError};

pub struct UpdateAuthConfigArgs<'a> {
    pub id: &'a str,
    pub client_id: Option<String>,
    pub client_secret: Option<Secret>,
    pub scopes: Option<Vec<String>>,
    /// Replaces the provider's own settings, when given.
    pub settings: Option<serde_json::Value>,
    /// Replaces the provider's own secret settings (JSON text), when given.
    pub secret_settings: Option<Secret>,
}

impl UseCases {
    pub async fn update_auth_config(
        &self,
        args: UpdateAuthConfigArgs<'_>,
    ) -> UseCaseResult<AuthConfig> {
        self.run("update_auth_config", async {
            if args
                .client_id
                .as_deref()
                .is_some_and(|id| id.trim().is_empty())
                || args
                    .client_secret
                    .as_ref()
                    .is_some_and(|secret| secret.expose_secret().trim().is_empty())
            {
                return Err(DomainError::Invalid(
                    "client_id and client_secret cannot be blank".into(),
                )
                .into());
            }
            let ctx = self.context();
            if args.settings.is_some() || args.secret_settings.is_some() {
                // The provider checks the settings the app will have, the new with the kept.
                let current = ctx.db.auth_config(args.id).await?;
                let provider = ctx
                    .providers
                    .get(&current.provider)
                    .ok_or(DomainError::Internal)?;
                let settings = args.settings.as_ref().unwrap_or(&current.settings);
                let secret_settings = args
                    .secret_settings
                    .as_ref()
                    .or(current.secret_settings.as_ref());
                validated_settings(provider, settings, secret_settings)?;
            }
            let updated = ctx
                .db
                .update_auth_config(
                    args.id,
                    AuthConfigChanges {
                        client_id: args.client_id,
                        client_secret: args.client_secret,
                        scopes: args.scopes,
                        settings: args.settings,
                        secret_settings: args.secret_settings,
                    },
                )
                .await?;
            // Tokens minted with the app's old credentials are dropped once the new ones are
            // saved, so none minted in between with the old ones stays.
            ctx.installation_tokens.forget_auth_config(args.id);
            Ok(updated)
        })
        .await
    }
}
