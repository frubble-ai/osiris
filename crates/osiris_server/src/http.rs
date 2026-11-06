//! What the HTTP processes share: the wrapper every route goes through, the keys, errors, versions,
//! idempotency, state and shutdown.

pub mod auth;
pub mod error;
pub mod health_answer;
pub mod idempotency;
pub mod serve_until_stopped;
pub mod server_wrap;
pub mod shutdown;
pub mod state;
pub mod version;

pub use server_wrap::server_wrap;
