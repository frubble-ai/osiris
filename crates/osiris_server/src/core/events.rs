//! Sending events to clients. An event is published right after the change that caused it; if
//! that publish fails, a task is scheduled to publish it again.

pub mod publish;
pub mod trigger;
