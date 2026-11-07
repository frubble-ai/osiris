//! The list of releases keeps its rules: dated and named in order, the last one current, only the
//! newest a preview, and every release after the first carrying the changes it made.

use crate::services::api::versioned_spec;
use osiris_api_models::version::{CURRENT, RELEASES, Stability};

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

#[test]
fn names_each_release_and_its_stability_in_its_spec() {
    for release in RELEASES {
        let spec = versioned_spec(RELEASES, release).unwrap();
        assert_eq!(spec["info"]["version"], release.name);
        assert_eq!(
            spec["info"]["x-osiris-stability"],
            release.stability.as_str()
        );
        assert_eq!(spec["info"]["x-osiris-release-number"], release.number);
    }
}

#[test]
fn names_the_current_release_in_the_current_spec() {
    let spec = crate::services::api::spec().unwrap();
    assert_eq!(spec["info"]["version"], CURRENT.name);
    assert_eq!(spec, versioned_spec(RELEASES, CURRENT).unwrap());
}

#[test]
fn describes_every_version_change_in_its_release_s_changelog() {
    for release in RELEASES {
        let path = format!(
            "{}/../../docs/changelog/{}.md",
            env!("CARGO_MANIFEST_DIR"),
            release.god()
        );
        let changelog = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("{release:?} has no changelog at {path}"));
        for change in release.changes {
            let description = change.description();
            assert!(
                !description.is_empty(),
                "{release:?} has a change without a description"
            );
            assert!(
                changelog.contains(description),
                "{release:?}'s changelog ({path}) must say what this change says: {description}"
            );
        }
    }
}
