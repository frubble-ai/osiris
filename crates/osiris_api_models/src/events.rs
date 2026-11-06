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
