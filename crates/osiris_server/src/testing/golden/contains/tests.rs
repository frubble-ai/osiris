//! `contains`: what a release recorded must still be there, unchanged; what was added since passes.

use super::contains;
use rstest::rstest;
use serde_json::{Value, json};

fn recorded() -> Value {
    json!({ "status": 200, "body": { "id": "<ac_1>", "scopes": ["repo"], "nested": { "n": 1 } } })
}

#[rstest]
#[case::the_same(recorded())]
#[case::a_field_added(json!({ "status": 200, "body": { "id": "<ac_1>", "scopes": ["repo"], "nested": { "n": 1, "m": 2 }, "label": "new" } }))]
fn passes_what_still_holds_the_recording(#[case] actual: Value) {
    assert_eq!(contains(&recorded(), &actual), Vec::<String>::new());
}

#[rstest]
#[case::a_changed_value(json!({ "status": 201, "body": { "id": "<ac_1>", "scopes": ["repo"], "nested": { "n": 1 } } }), "/status")]
#[case::a_removed_field(json!({ "status": 200, "body": { "id": "<ac_1>", "nested": { "n": 1 } } }), "/body/scopes")]
#[case::a_longer_list(json!({ "status": 200, "body": { "id": "<ac_1>", "scopes": ["repo", "org"], "nested": { "n": 1 } } }), "/body/scopes")]
#[case::a_changed_nested_value(json!({ "status": 200, "body": { "id": "<ac_1>", "scopes": ["repo"], "nested": { "n": 2 } } }), "/body/nested/n")]
#[case::a_changed_type(json!({ "status": "200", "body": { "id": "<ac_1>", "scopes": ["repo"], "nested": { "n": 1 } } }), "/status")]
fn names_where_the_recording_no_longer_holds(#[case] actual: Value, #[case] at: &str) {
    let found = contains(&recorded(), &actual);

    assert_eq!(found.len(), 1, "{found:?}");
    assert!(found[0].starts_with(at), "{found:?}");
}
