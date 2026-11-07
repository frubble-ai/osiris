//! A release's `OpenAPI` document: the current one, taken back through the changes made since.

use super::spec::spec;
use osiris_api_models::version::{Release, translate::spec_down};
use serde_json::Value;

/// `release`'s spec, one of `releases`.
pub fn versioned_spec(releases: &[Release], release: &Release) -> Value {
    let mut spec = spec();
    spec_down(releases, release, &mut spec);
    spec
}
