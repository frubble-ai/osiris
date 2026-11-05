//! Idempotency keys: a POST or DELETE runs once per key; a retry gets the first answer.

pub mod abandon;
pub mod begin;
pub mod finish;

pub use osiris_storage::idempotency::Begun;
