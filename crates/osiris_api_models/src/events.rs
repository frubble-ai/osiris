//! `/v1/events/trigger`: a test event, for a client to try its handling.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct TriggerEvent {
    /// `connection.active`, `connection.expired` or `connection.revoked`.
    #[serde(rename = "type")]
    pub event_type: String,
    pub connected_account_id: String,
}

/// The event as it was sent.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct TriggeredEvent {
    pub event_id: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub connected_account_id: String,
}

/// An event as a subscriber receives it on the events topic, written in the API key's pinned
/// release. The type and the connection's id, never its data: fetch the connection for its state.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct EventDelivery {
    /// The envelope's format.
    pub version: u32,
    /// `msg_…`, this delivery's own id.
    pub id: String,
    #[schema(value_type = String, format = DateTime)]
    pub occurred_at: String,
    /// The API release the event is written in, also the message's `osiris_version` attribute.
    pub osiris_version: String,
    /// `ev_…`, the same each time the event is delivered, so a client can drop repeats.
    pub event_id: String,
    /// What happened, also the message's `type` attribute: `connection.active`,
    /// `connection.expired` or `connection.revoked` today. An open string: new types may come
    /// without a new release, and a client ignores a type it doesn't know.
    #[serde(rename = "type")]
    pub event_type: String,
    pub connected_account_id: String,
}
