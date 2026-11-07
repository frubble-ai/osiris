//! A release's contract: its `OpenAPI` document without the words written for people (titles,
//! summaries, descriptions, examples, the document's `info` and its tags' descriptions). Rewording
//! the documentation leaves the contract as it is; changing a path, a field, a type, a status, a
//! required list or an enum changes it.

use serde_json::{Map, Value};

/// The words for people, dropped wherever they annotate something.
const ANNOTATIONS: &[&str] = &[
    "title",
    "summary",
    "description",
    "example",
    "examples",
    "externalDocs",
];

pub fn contract(spec: &Value) -> Value {
    let mut spec = spec.clone();
    if let Some(document) = spec.as_object_mut() {
        document.remove("info");
        document.remove("tags");
    }
    strip(&spec, false)
}

/// `names` is true inside a map whose keys are names (a schema's `properties`), where `title` or
/// `example` is a field and stays.
fn strip(value: &Value, names: bool) -> Value {
    match value {
        Value::Object(map) => {
            let kept: Map<String, Value> = map
                .iter()
                .filter(|(key, _)| names || !ANNOTATIONS.contains(&key.as_str()))
                .map(|(key, child)| (key.clone(), strip(child, !names && key == "properties")))
                .collect();
            Value::Object(kept)
        }
        Value::Array(items) => Value::Array(items.iter().map(|item| strip(item, false)).collect()),
        other => other.clone(),
    }
}
