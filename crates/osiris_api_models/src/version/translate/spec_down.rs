//! The current `OpenAPI` document taken back to `target`'s, its version named in `info.version`,
//! its stability in `info.x-osiris-stability` and its number in `info.x-osiris-release-number`.

use super::changes_since;
use crate::version::Release;
use serde_json::Value;

/// Takes `spec` back through each change since `target`, newest first, and names `target` in its
/// `info`.
pub fn spec_down(releases: &[Release], target: &Release, spec: &mut Value) {
    for change in changes_since(releases, target).rev() {
        change.spec(spec);
    }
    if let Some(info) = spec.get_mut("info").and_then(Value::as_object_mut) {
        info.insert("version".into(), Value::String(target.name.into()));
        info.insert(
            "x-osiris-stability".into(),
            Value::String(target.stability.as_str().into()),
        );
        info.insert("x-osiris-release-number".into(), Value::from(target.number));
    }
}
