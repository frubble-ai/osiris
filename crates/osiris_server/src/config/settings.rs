//! The settings, one type per section. Each section's `Default` holds its defaults;
//! `config.example.toml` documents every key.

pub mod all;
pub mod api;
pub mod consent;
pub mod database;
pub mod http;
pub mod idempotency;
pub mod pagination;
pub mod positive_values;
pub mod providers;
pub mod refresh;
pub mod seconds;
pub mod server;
pub mod tasks;
pub mod triggers;
pub mod validate;

pub use all::Settings;
pub(crate) use seconds::seconds;
