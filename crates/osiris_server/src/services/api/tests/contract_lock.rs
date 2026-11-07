//! Each release is locked to its contract: the spec osiris generates for it today must have the
//! digest written beside its name. No change to a contract goes unnoticed: an addition updates the
//! digest in the same pull request, where CI's breaking-change gate holds it to breaking nothing,
//! and a breaking change comes as a new release whose version changes give the old one its spec
//! back.

use crate::services::api::versioned_spec;
use osiris_api_models::version::{CURRENT, RELEASES, contract_sha256};

#[test]
fn keeps_every_release_s_contract_as_it_was_published() {
    for release in RELEASES {
        let digest = contract_sha256(&versioned_spec(RELEASES, release));
        assert_eq!(
            digest, release.contract_sha256,
            "{}'s contract changed, and its digest is now {digest}. An addition (a field, an \
             endpoint, an answer) only updates the digest in releases.rs, and CI checks it breaks \
             nothing. A breaking change is a new release instead (docs/api-versioning.md), whose \
             version changes give {} its old spec back.",
            release.name, release.name,
        );
    }
}

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
