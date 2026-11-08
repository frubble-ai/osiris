//! A fired trigger's data, kept under a size every event and its retry task can carry. A message
//! bus or a task queue caps what one message holds (a task far less than a delivery may be), so
//! data larger than `[triggers] max_event_data_bytes` is replaced by a note of its size, and the
//! client reads what changed from the provider itself.

use serde_json::{Value, json};

/// `data`, or, when its JSON is longer than `max_bytes`, `{"omitted": true, "bytes": <length>}`.
pub fn bounded_trigger_data(data: Value, max_bytes: usize) -> Value {
    let length = serde_json::to_vec(&data).map_or(usize::MAX, |bytes| bytes.len());
    if length <= max_bytes {
        return data;
    }
    tracing::warn!(
        bytes = length,
        max_bytes,
        "a fired trigger's data is too large to carry; it is omitted"
    );
    json!({ "omitted": true, "bytes": length })
}

#[cfg(test)]
mod tests;
