//! Implementations that need no outside service: the local scheduler for development, the
//! in-memory one for tests, and the bearer-token authenticator they use. The Cloud Tasks scheduler
//! and its OIDC authenticator are in the `osiris_cloud_tasks` crate.

pub mod bearer;
pub mod local;
pub mod memory;
