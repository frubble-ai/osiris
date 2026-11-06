//! The worker: an internal HTTP service that the task scheduler calls with each scheduled task
//! when it is due (`POST /tasks`). It does nothing between calls, so it can scale to zero. After
//! SIGTERM, `/health` answers `draining` while tasks already started finish.

pub mod authenticator;
pub mod health;
pub mod router;
pub mod serve;
pub mod state;
pub mod tasks;

pub use router::router;
pub use serve::serve;

#[cfg(test)]
mod tests;
