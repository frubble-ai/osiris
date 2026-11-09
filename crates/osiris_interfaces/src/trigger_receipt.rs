//! What a provider makes of a delivery sent to a trigger.

use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum TriggerReceipt {
    /// Not signed with the trigger's secret: refused, and nothing is said of the trigger.
    Unsigned,
    /// Signed, but not something the trigger fires on (a ping, an event it doesn't watch); why.
    Ignored(String),
    /// The trigger fires: the provider's id of the delivery, to drop repeats, and what happened.
    Fired {
        delivery_id: Option<String>,
        data: Value,
    },
    /// Something may have changed: the trigger checks the provider now, as it does on schedule.
    Check,
}
