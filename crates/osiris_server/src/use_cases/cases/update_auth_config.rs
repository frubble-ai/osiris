//! Changes an app's client, secret or scopes. Connections made before keep the scopes they got.

use crate::use_cases::lib::auth_configs::AuthConfigChanges;
use crate::use_cases::{UseCaseResult, UseCases};
use osiris_common::secret::{ExposeSecret, Secret};
use osiris_domain::{AuthConfig, DomainError};

pub struct UpdateAuthConfigArgs<'a> {
    pub id: &'a str,
    pub client_id: Option<String>,
    pub client_secret: Option<Secret>,
    pub scopes: Option<Vec<String>>,
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
            self.context()
                .db
                .update_auth_config(
                    args.id,
                    AuthConfigChanges {
                        client_id: args.client_id,
                        client_secret: args.client_secret,
                        scopes: args.scopes,
                    },
                )
                .await
        })
        .await
    }
}
