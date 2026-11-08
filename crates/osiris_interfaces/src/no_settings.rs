//! Whether an app carries no settings of its own: the rule every provider's `validate_settings`
//! starts from.

use serde_json::Value;

/// Whether `settings` and `secret_settings` are both absent or empty objects.
pub fn no_settings(settings: &Value, secret_settings: Option<&Value>) -> bool {
    let empty = |value: &Value| value.as_object().is_none_or(serde_json::Map::is_empty);
    empty(settings) && secret_settings.is_none_or(empty)
}
