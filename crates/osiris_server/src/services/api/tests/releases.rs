//! Each release's spec names it, and each release's changelog says what its changes changed. The
//! release list's own rules are tested beside it, in `osiris_versioning`.

use crate::services::api::versioned_spec;
use osiris_versioning::{CURRENT, RELEASES};

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
