//! What osiris does, whichever process runs it. The API's routes and the worker's task handler
//! both call into this module, and only this module uses storage, the providers, the event bus
//! and the task scheduler, each through its own interface. One domain per module, one operation
//! per file.

pub mod actions;
pub mod auth_configs;
pub mod connected_accounts;
pub mod context;
pub mod errors;
pub mod events;
pub mod health;
pub mod idempotency;
pub mod oauth_callback;
pub mod oauth_client;
pub mod refresh;
pub mod tasks;

pub use context::Core;
pub use errors::CoreResult;
