//! `idempotency_key`: a POST or DELETE with an `Idempotency-Key` runs once; a retry with the same
//! key gets the first answer again. This lasts for the retention period: after that, the key is
//! treated as unused. Expired keys are deleted a few at a time whenever a new key is written. One
//! operation per module.

pub mod abandon;
pub mod begin;
pub mod finish;

pub use osiris_domain::Begun;
