//! The current release's `OpenAPI` document, generated from the routes, with the answers every
//! `/v1` route can give before it runs (its ground rules) added to each operation, and the events
//! clients receive as its webhooks. Its `info` names the current release, from `RELEASES`, so the
//! release's name is written in one place.

use super::{events_webhook::events_webhook, ground_rule_answers::ground_rule_answers, v1::v1};
use osiris_versioning::{RELEASES, translate::spec_down};
use serde_json::Value;

/// The current release's spec, as `api-reference/openapi.json` holds it. Fails only if the
/// document can't be written as JSON, which `osiris_openapi` reports rather than writing `null`.
pub fn spec() -> serde_json::Result<Value> {
    let mut spec = serde_json::to_value(v1().split_for_parts().1)?;
    ground_rule_answers(&mut spec);
    events_webhook(&mut spec);
    if let Some(current) = RELEASES.last() {
        spec_down(RELEASES, current, &mut spec);
    }
    Ok(spec)
}
