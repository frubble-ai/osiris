//! What the API takes and answers, one module per resource: the types the `OpenAPI` spec and the
//! client package are generated from. Secrets never appear in any of them.

pub mod actions;
pub mod auth_configs;
pub mod connected_accounts;
pub mod errors;
pub mod events;
pub mod list;
pub mod secret_settings;
pub mod triggers;
