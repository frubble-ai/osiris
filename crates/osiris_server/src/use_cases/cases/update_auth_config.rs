//! Changes an app's client, secret or scopes. Connections made before keep the scopes they got.

use crate::use_cases::lib::auth_configs::AuthConfigChanges;
use crate::use_cases::{UseCaseResult, UseCases};
use osiris_common::secret::Secret;
use osiris_domain::AuthConfig;

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
