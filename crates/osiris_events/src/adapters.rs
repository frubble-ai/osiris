//! The in-memory transport, for tests. The Pub/Sub transport is in the `osiris_pubsub` crate.

pub mod memory;

#[cfg(test)]
mod mocks;
#[cfg(test)]
mod tests;
