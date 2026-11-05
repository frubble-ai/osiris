//! Osiris's messaging, as the app's `packages/events`: a catalog of every message, the envelope
//! each travels in, the bus use cases send through, the ports, and the adapters behind them
//! (an in-memory one here for tests, and Pub/Sub in the `osiris_pubsub` crate, chosen when a
//! process starts). The core sends through the bus and does not know which transport is used.
//!
//! There is one channel, `events`, which carries stateless events to clients. Work osiris must do
//! at a set time is not sent as a message; it is a scheduled task (see `osiris_tasks`).

pub mod adapters;
pub mod catalog;
pub mod envelope;
pub mod message_bus;
pub mod ports;
#[cfg(any(test, feature = "testing"))]
pub mod testing;

pub use catalog::{Channel, Message};
pub use envelope::Envelope;
pub use message_bus::MessageBus;
pub use ports::{Transport, TransportError};
