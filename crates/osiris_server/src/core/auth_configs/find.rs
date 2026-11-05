//! One provider's app.

use crate::core::{Core, CoreResult};
use osiris_common::errors::ReportSwitchExt;
use osiris_domain::AuthConfig;

impl Core {
    pub async fn auth_config(&self, id: &str) -> CoreResult<AuthConfig> {
        self.store.auth_config(id).await.switch()
    }
}
