//! The current `OpenAPI` document taken back to `target`'s, its version named in `info.version`.

use super::changes_since;
use crate::version::Release;
use serde_json::Value;

pub fn spec_down(releases: &[Release], target: &Release, spec: &mut Value) {
    for change in changes_since(releases, target).rev() {
        change.spec(spec);
    }
    if let Some(info) = spec.get_mut("info").and_then(Value::as_object_mut) {
        info.insert("version".into(), Value::String(target.name.into()));
    }
}
