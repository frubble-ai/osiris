//! What the API adds to the core: its two keys.

use super::{CoreConfig, Settings, env_vars::secret};
use osiris_common::secret::Secret;

#[derive(Clone)]
pub struct ApiConfig {
    pub core: CoreConfig,
    /// What a client sends to use connections and actions.
    pub api_key: Secret,
    /// What an operator sends to register providers' apps, refresh connections and trigger events; never given to a client.
    pub admin_key: Secret,
}

impl ApiConfig {
    pub fn from_env(settings: Settings) -> anyhow::Result<Self> {
        Ok(Self {
            core: CoreConfig::from_env(settings)?,
            api_key: secret("OSIRIS_API_KEY")?,
            admin_key: secret("OSIRIS_ADMIN_KEY")?,
        })
    }
}
