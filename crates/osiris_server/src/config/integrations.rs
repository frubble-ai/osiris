//! The integrations' configuration, one module per concern: where events go (`events`) and what
//! runs scheduled tasks (`tasks`). Each reads its own variables; the use cases never see a vendor.

pub mod events;
pub mod integrations_config;
pub mod tasks;

pub use integrations_config::IntegrationsConfig;
pub use tasks::TasksConfig;
