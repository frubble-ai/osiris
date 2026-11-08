//! The repository webhook a trigger asks GitHub for: JSON deliveries of `events` to
//! `callback_url`, signed with `secret`.

use serde_json::{Value, json};

/// The body GitHub's hooks API takes to create or align the webhook.
pub fn hook_body(callback_url: &str, secret: &str, events: &[String], insecure_ssl: bool) -> Value {
    json!({
        "name": "web",
        "active": true,
        "events": events,
        "config": {
            "url": callback_url,
            "content_type": "json",
            "secret": secret,
            "insecure_ssl": if insecure_ssl { "1" } else { "0" },
        },
    })
}
