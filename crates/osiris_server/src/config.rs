//! Configuration. Settings (timings, limits) come layered from `config/` and `OSIRIS__…`
//! variables; secrets and infrastructure come from environment variables of their own. Both are
//! read once at startup, and a missing or malformed value stops the process naming it.

pub mod api_config;
pub mod app_config;
pub mod database_config;
pub mod env_vars;
pub mod listen;
pub mod load;
pub mod process;
pub mod settings;
pub mod tasks_config;

pub use api_config::ApiConfig;
pub use app_config::Config;
pub use database_config::DatabaseConfig;
pub use settings::Settings;
pub use tasks_config::TasksConfig;
