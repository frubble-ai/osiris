//! For tests: the contract every `Transport` is held to, and generated events.

#![allow(
    clippy::expect_used,
    clippy::missing_panics_doc,
    reason = "a contract's failure is the test failing"
)]

pub mod generate_event_message;
pub mod transport_contract;

pub use transport_contract::{TransportHarness, transport_contract};
