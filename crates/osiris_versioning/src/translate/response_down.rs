//! An answer in the current release's shape, taken back to `target`.

use super::changes_since;
use crate::{Release, Route};
use serde_json::Value;

/// Takes `body`, answered with `status`, back through each change since `target`, newest first.
pub fn response_down(
    releases: &[Release],
    target: &Release,
    route: &Route<'_>,
    status: u16,
    body: &mut Value,
) {
    for change in changes_since(releases, target).rev() {
        change.response(route, status, body);
    }
}
