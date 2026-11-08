//! Triggers: a connection's subscriptions to something happening at its provider, created and
//! deleted there, their deliveries read and published as `trigger.fired` events, and, for a kind
//! that checks on a schedule, their checks.

pub mod check_trigger;
pub mod create_trigger;
pub mod delete_trigger;
pub mod get_trigger;
pub mod list_triggers;
pub mod receive_trigger_delivery;
