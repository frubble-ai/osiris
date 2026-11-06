//! The configuration the use cases run on, read from the environment once at boot: the settings,
//! the database, the master key, osiris's public URL, and the integrations (where events are
//! published, which scheduler runs tasks). It reaches the use cases as `ctx.config`.

use super::{
    DatabaseConfig, Settings,
    env_vars::{required, secret},
    integrations::IntegrationsConfig,
};
use osiris_common::secret::Secret;

#[derive(Clone)]
pub struct Config {
    pub settings: Settings,
    pub database: DatabaseConfig,
    /// The master key every tenant's data key is wrapped with.
    pub encryption_key: Secret,
    /// Osiris's own public origin, where providers send the browser back (`/oauth/callback`).
    pub public_url: String,
    /// Where events are published, and what runs scheduled tasks.
    pub integrations: IntegrationsConfig,
}

impl Config {
    pub fn from_env(settings: Settings) -> anyhow::Result<Self> {
        Ok(Self {
            database: DatabaseConfig::from_env(&settings)?,
            encryption_key: secret("OSIRIS_ENCRYPTION_KEY")?,
            public_url: required("OSIRIS_PUBLIC_URL")?
                .trim_end_matches('/')
                .to_owned(),
            integrations: IntegrationsConfig::from_env()?,
            settings,
        })
    }
}
