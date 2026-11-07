//! The changes made after `target` up to the last of `releases`, oldest first.

use crate::{Release, VersionChange};

/// Every change the releases after `target` made, oldest first; none when `target` is the last.
pub fn changes_since<'a>(
    releases: &'a [Release],
    target: &Release,
) -> impl DoubleEndedIterator<Item = &'a dyn VersionChange> {
    let after = releases
        .iter()
        .position(|release| release.name == target.name)
        .map_or(releases.len(), |index| index + 1);
    releases
        .get(after..)
        .unwrap_or_default()
        .iter()
        .flat_map(|release| release.changes.iter().copied())
}
