//! Whether what osiris does now still holds what was recorded: every recorded field is there with
//! the same value, lists keep their length, and a field osiris added since is allowed.

use serde_json::Value;

/// Where `actual` departs from `recorded`, one line each; empty when it holds it.
pub fn contains(recorded: &Value, actual: &Value) -> Vec<String> {
    let mut found = Vec::new();
    walk(recorded, actual, "", &mut found);
    found
}

/// Adds to `found` each place under `at` where `actual` departs from `recorded`.
fn walk(recorded: &Value, actual: &Value, at: &str, found: &mut Vec<String>) {
    match (recorded, actual) {
        (Value::Object(recorded), Value::Object(actual)) => {
            for (key, value) in recorded {
                let here = format!("{at}/{key}");
                match actual.get(key) {
                    Some(now) => walk(value, now, &here, found),
                    None => found.push(format!("{here}: was {value}, is gone")),
                }
            }
        }
        (Value::Array(recorded), Value::Array(actual)) => {
            if recorded.len() == actual.len() {
                for (index, (value, now)) in recorded.iter().zip(actual).enumerate() {
                    walk(value, now, &format!("{at}/{index}"), found);
                }
            } else {
                found.push(format!(
                    "{at}: held {} items, holds {}",
                    recorded.len(),
                    actual.len()
                ));
            }
        }
        (recorded, actual) if recorded == actual => {}
        (recorded, actual) => found.push(format!("{at}: was {recorded}, is {actual}")),
    }
}

#[cfg(test)]
mod tests;
