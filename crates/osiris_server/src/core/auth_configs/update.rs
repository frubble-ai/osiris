//! Change an app's client, secret or scopes. Connections made before keep the scopes they got.

use crate::core::{Core, CoreResult};
use osiris_common::{errors::ReportSwitchExt, secret::Secret};
use osiris_domain::AuthConfig;
use osiris_storage::auth_configs::AuthConfigChanges;

#[derive(Default)]
pub struct UpdateAuthConfig {
    pub client_id: Option<String>,
    pub client_secret: Option<Secret>,
    pub scopes: Option<Vec<String>>,
}

impl Core {
    pub async fn update_auth_config(
        &self,
        id: &str,
        changes: UpdateAuthConfig,
    ) -> CoreResult<AuthConfig> {
        self.store
            .update_auth_config(
                id,
                AuthConfigChanges {
                    client_id: changes.client_id,
                    client_secret: changes.client_secret,
                    scopes: changes.scopes,
                },
            )
            .await
            .switch()
    }
}
