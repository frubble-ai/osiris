//! A new preview never becomes the default pin while a stable release exists.

use super::pinned_by_default_among;
use crate::{Release, Stability, mocks::MOCK_RELEASES};

#[test]
fn pins_the_newest_stable_release_not_a_newer_preview() {
    // amun and anubis are stable, bastet a preview.
    let pinned = pinned_by_default_among(MOCK_RELEASES).unwrap();
    assert_eq!(pinned.name, "2026-02-01.anubis");
}

#[test]
fn pins_the_only_release_while_none_is_stable() {
    let first = [Release {
        name: "2026-11-04.anubis",
        number: 1,
        stability: Stability::Preview,
        changes: &[],
    }];
    assert_eq!(
        pinned_by_default_among(&first).unwrap().name,
        "2026-11-04.anubis"
    );
    assert!(pinned_by_default_among(&[]).is_none());
}
