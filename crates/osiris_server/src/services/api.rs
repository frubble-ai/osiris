//! The HTTP API: `/v1` behind the version header and idempotency keys, every route through
//! `server_wrap`; the OAuth callback and `/health` in the open.

pub mod doc;
pub mod router;
pub mod serve;
pub mod spec;
pub mod state;
pub mod v1;

pub use router::router;
pub use serve::serve;
pub use spec::spec;
pub use state::state;

#[cfg(test)]
mod tests;
