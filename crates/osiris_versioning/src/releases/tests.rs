//! The list of releases keeps its rules: dated, named and numbered in order, the last one current,
//! only the newest a preview, and every release after the first carrying the changes it made.

use crate::{CURRENT, RELEASES, Stability};

#[test]
fn names_releases_in_order_and_serves_the_last_as_the_current_one() {
    let last = RELEASES.last().expect("at least one release");
    assert_eq!(last.name, CURRENT.name);
    for pair in RELEASES.windows(2) {
        let (older, newer) = (&pair[0], &pair[1]);
        assert!(
            older.released_on() < newer.released_on(),
            "{older:?} must come out before {newer:?}"
        );
        assert!(
            older.god() < newer.god(),
            "gods go in alphabetical order: {older:?} before {newer:?}"
        );
        assert!(
            !newer.changes.is_empty(),
            "{newer:?} must carry the changes it made"
        );
        assert_eq!(
            newer.number,
            older.number + 1,
            "{newer:?} must be numbered after {older:?}"
        );
    }
    for release in RELEASES {
        assert!(
            release.released_on().is_some(),
            "{release:?} must start with its date"
        );
        assert!(!release.god().is_empty(), "{release:?} must name its god");
    }
    assert!(
        RELEASES[0].changes.is_empty(),
        "the first release changes nothing"
    );
}

#[test]
fn keeps_every_release_but_the_newest_stable() {
    let (_, older) = RELEASES.split_last().expect("at least one release");
    for release in older {
        assert_eq!(
            release.stability,
            Stability::Stable,
            "{release:?} has a newer release, so it must be stable: only the newest is a preview"
        );
    }
}
