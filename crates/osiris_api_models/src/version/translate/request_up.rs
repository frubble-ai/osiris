//! A request's body written for `target`, brought up to the current release.

use super::changes_since;
use crate::version::{Release, Route};
use serde_json::Value;

pub fn request_up(releases: &[Release], target: &Release, route: &Route<'_>, body: &mut Value) {
    for change in changes_since(releases, target) {
        change.request(route, body);
    }
}
