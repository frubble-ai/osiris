//! A `trigger.fired` event's id: the same for the same delivery or change of the same trigger, so
//! a provider's redelivery (or a check seeing a change a delivery already reported) is the same
//! event, which a client drops by its id.

use sha2::{Digest, Sha256};

/// `ev_` and a digest of the trigger and what it saw (the provider's delivery id, or the check's
/// change id).
pub fn trigger_event_id(trigger_id: &str, seen: &str) -> String {
    let digest = Sha256::digest(format!("{trigger_id}\n{seen}").as_bytes());
    format!("ev_{}", hex::encode(digest).get(..32).unwrap_or_default())
}
