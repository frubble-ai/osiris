//! The API's routes: one module per resource, one file per operation. A handler parses its request
//! and names the use case `server_wrap` runs; nothing else.

pub mod actions;
pub mod auth_configs;
pub mod connected_accounts;
pub mod cursor;
pub mod events;
pub mod health;
pub mod list;
pub mod oauth_callback;
