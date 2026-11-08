//! A release's `OpenAPI` document: the current one, taken back through the changes made since. A
//! superseded release's spec also records its sunset, `info.x-osiris-sunset`, at the support
//! window osiris ships with (`[api] support_days`'s default): what its clients were promised, which
//! the contract guard holds a removal to, and which may never be shortened.

use super::spec::spec;
use crate::config::settings::api::ApiSettings;
use osiris_versioning::{Release, superseded, translate::spec_down};
use serde_json::Value;
use time::macros::format_description;

/// `release`'s spec, one of `releases`.
pub fn versioned_spec(releases: &[Release], release: &Release) -> serde_json::Result<Value> {
    let mut spec = spec()?;
    spec_down(releases, release, &mut spec);
    let sunset =
        superseded(releases, release, ApiSettings::default().support()).and_then(|(_, sunset)| {
            sunset
                .format(format_description!("[year]-[month]-[day]"))
                .ok()
        });
    if let (Some(sunset), Some(info)) =
        (sunset, spec.get_mut("info").and_then(Value::as_object_mut))
    {
        info.insert("x-osiris-sunset".into(), Value::String(sunset));
    }
    Ok(spec)
}
