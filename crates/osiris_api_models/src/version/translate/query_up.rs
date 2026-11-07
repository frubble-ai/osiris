//! A request's query written for `target`, brought up to the current release.

use super::changes_since;
use crate::version::{Release, Route};

/// Brings `query` up through each change since `target`, oldest first.
pub fn query_up(
    releases: &[Release],
    target: &Release,
    route: &Route<'_>,
    query: &mut Vec<(String, String)>,
) {
    for change in changes_since(releases, target) {
        change.query(route, query);
    }
}
