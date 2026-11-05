//! What every process needs: the settings, the database, the master key, where events are
//! published, and which scheduler runs tasks.

use super::{
    DatabaseConfig, Settings, TasksConfig,
    env_vars::{optional, required, secret},
};
use osiris_common::secret::Secret;
use osiris_pubsub::PubSubConfig;

#[derive(Clone)]
pub struct CoreConfig {
    pub settings: Settings,
    pub database: DatabaseConfig,
    /// The master key every tenant's data key is wrapped with.
    pub encryption_key: Secret,
    /// Osiris's own public origin, where providers send the browser back (`/oauth/callback`).
    pub public_url: String,
    /// Where events for clients are published.
    pub events: PubSubConfig,
    pub tasks: TasksConfig,
}

impl CoreConfig {
    pub fn from_env(settings: Settings) -> anyhow::Result<Self> {
        Ok(Self {
            database: DatabaseConfig::from_env(&settings)?,
            encryption_key: secret("OSIRIS_ENCRYPTION_KEY")?,
            public_url: required("OSIRIS_PUBLIC_URL")?
                .trim_end_matches('/')
                .to_owned(),
            events: PubSubConfig {
                project: required("PUBSUB_PROJECT")?,
                events_topic: required("OSIRIS_EVENTS_TOPIC")?,
                emulator_host: optional("PUBSUB_EMULATOR_HOST"),
            },
            tasks: TasksConfig::from_env()?,
            settings,
        })
    }
}
