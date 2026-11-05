//! Every integration's configuration: where events go and what runs scheduled tasks.

use super::{TasksConfig, events::events_config_from_env};
use osiris_pubsub::PubSubConfig;

/// Every integration's configuration.
#[derive(Clone)]
pub struct IntegrationsConfig {
    pub events: PubSubConfig,
    pub tasks: TasksConfig,
}

impl IntegrationsConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            events: events_config_from_env()?,
            tasks: TasksConfig::from_env()?,
        })
    }
}
