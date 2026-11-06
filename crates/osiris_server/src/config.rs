//! Configuration. Settings (timings, limits) are the defaults in code, overridden by
//! `OSIRIS__<SECTION>__<KEY>` variables; secrets and infrastructure come from environment
//! variables of their own. Both are read once at startup, and a missing or malformed value stops
//! the process naming it.

pub mod api_config;
pub mod app_config;
pub mod database_config;
pub mod env_vars;
pub mod integrations;
pub mod listen;
pub mod load;
pub mod migrate;
pub mod process;
pub mod settings;

pub use api_config::ApiConfig;
pub use app_config::Config;
pub use database_config::DatabaseConfig;
pub use integrations::TasksConfig;
pub use migrate::MigrateConfig;
pub use settings::Settings;
