//! What a provider saw when it checked a trigger that checks on a schedule.

use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct TriggerCheck {
    /// What to compare the next check with.
    pub checkpoint: Value,
    /// What changed since the last check, when something did: the event's `data`, and an id
    /// that is the same for the same change, to drop repeats.
    pub fired: Option<(String, Value)>,
}
