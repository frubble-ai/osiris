//! A release's spec, generated once per set of releases and release and shared by every check,
//! since every request a test sends is checked against it.

use crate::services::api::versioned_spec;
use osiris_api_models::version::Release;
use serde_json::Value;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock, PoisonError},
};

type Cache = Mutex<HashMap<String, Arc<Value>>>;

/// The key a release's spec is kept under: the releases served, by name, and the release.
pub fn release_key(releases: &[Release], release: &Release) -> String {
    let served: Vec<&str> = releases.iter().map(|listed| listed.name).collect();
    format!("{} at {}", release.name, served.join(","))
}

/// `release`'s spec among `releases`, generated on first use.
pub fn release_spec(releases: &[Release], release: &Release) -> Arc<Value> {
    static CACHE: OnceLock<Cache> = OnceLock::new();
    let key = release_key(releases, release);
    let cache = CACHE.get_or_init(Cache::default);
    if let Some(spec) = cache
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .get(&key)
    {
        return spec.clone();
    }
    let spec = Arc::new(versioned_spec(releases, release));
    cache
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .insert(key, spec.clone());
    spec
}
