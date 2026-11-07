//! A validator for the schema at `pointer` in a release's spec, built once per release and
//! pointer. The spec itself is the root, so the schema's `$ref`s to `#/components/schemas/…`
//! resolve.

use super::release_spec::{release_key, release_spec};
use jsonschema::Validator;
use osiris_api_models::version::Release;
use serde_json::{Value, json};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock, PoisonError},
};

type Cache = Mutex<HashMap<(String, String), Arc<Validator>>>;

/// The validator for the schema at `pointer` in `release`'s spec, built once.
pub fn schema_validator(releases: &[Release], release: &Release, pointer: &str) -> Arc<Validator> {
    static CACHE: OnceLock<Cache> = OnceLock::new();
    let key = (release_key(releases, release), pointer.to_owned());
    let cache = CACHE.get_or_init(Cache::default);
    if let Some(validator) = cache
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .get(&key)
    {
        return validator.clone();
    }
    let mut root = (*release_spec(releases, release)).clone();
    if let Some(document) = root.as_object_mut() {
        // A `$ref` is a URI, so the braces of a path template are percent-encoded.
        let fragment = pointer.replace('{', "%7B").replace('}', "%7D");
        document.insert("$ref".into(), json!(format!("#{fragment}")));
    }
    let validator = Arc::new(
        jsonschema::draft202012::new(&root)
            .unwrap_or_else(|error| panic!("{release:?}'s schema at {pointer}: {error}")),
    );
    cache
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .insert(key, validator.clone());
    validator
}

/// `body`'s mismatches with the schema at `pointer`, one per line; empty when it matches.
pub fn mismatches(releases: &[Release], release: &Release, pointer: &str, body: &Value) -> String {
    schema_validator(releases, release, pointer)
        .iter_errors(body)
        .map(|error| format!("{} at {}", error, error.instance_path()))
        .collect::<Vec<_>>()
        .join("\n")
}
