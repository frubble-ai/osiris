//! The events topic, written into the spec as its one webhook (`OpenAPI` 3.1's `webhooks`), so the
//! events clients receive are part of each release's contract and its breaking-change checks, like any
//! answer.

use serde_json::{Value, json};

/// Adds the events topic to `spec` as its `events` webhook.
pub fn events_webhook(spec: &mut Value) {
    let Some(document) = spec.as_object_mut() else {
        return;
    };
    document.insert(
        "webhooks".into(),
        json!({
            "events": {
                "post": {
                    "operationId": "receive_event",
                    "summary": "An event on the events topic",
                    "description": "Published on the events topic, with the `type` and `osiris_version` attributes, written in the release the API key is pinned to.",
                    "requestBody": {
                        "required": true,
                        "content": { "application/json": { "schema": { "$ref": "#/components/schemas/EventDelivery" } } }
                    },
                    "responses": { "200": { "description": "Received" } }
                }
            }
        }),
    );
}
