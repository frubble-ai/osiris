//! The processes, all from one image. `serve` is the public HTTP API and the OAuth callback.
//! `worker` is the internal service the task scheduler calls with each scheduled task. Both only
//! answer requests, so both can scale to zero. `migrate` runs once per deploy. `build` assembles
//! the core and picks the adapters behind its interfaces.

pub mod api;
pub mod build;
pub mod worker;
