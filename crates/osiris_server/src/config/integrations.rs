//! The integrations' configuration, one module per concern: where events go (`events`) and what
//! runs scheduled tasks (`tasks`). Each reads its own variables; the use cases never see a vendor.

pub mod events;
pub mod tasks;

pub use tasks::TasksConfig;

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
            events: events::events_config_from_env()?,
            tasks: TasksConfig::from_env()?,
        })
    }
}
