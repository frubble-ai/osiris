//! An event in the current release's shape, taken back to `target`.

use super::changes_since;
use crate::version::Release;
use serde_json::Value;

/// Takes `event` back through each change since `target`, newest first.
pub fn event_down(releases: &[Release], target: &Release, event: &mut Value) {
    for change in changes_since(releases, target).rev() {
        change.event(event);
    }
}
