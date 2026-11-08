//! An event as it was published, checked against the `EventDelivery` schema of the release it
//! says it is written in.

use super::schema_validator::mismatches;
use osiris_versioning::Release;
use serde_json::Value;

/// Fails the test unless `event` matches `EventDelivery` in the spec of the release it names.
pub fn assert_event_conforms(releases: &[Release], event: &Value) {
    let named = event["osiris_version"].as_str().unwrap_or_default();
    let release = releases
        .iter()
        .find(|release| release.name == named)
        .unwrap_or_else(|| panic!("an event written in {named:?}, not a release served: {event}"));
    let found = mismatches(
        releases,
        release,
        "/components/schemas/EventDelivery",
        event,
    );
    assert!(
        found.is_empty(),
        "an event out of {release:?}'s spec:\n{found}\n{event}"
    );
}
