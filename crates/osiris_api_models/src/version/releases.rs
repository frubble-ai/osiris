//! Every release osiris serves, oldest first. The last is the current one: the shape the code
//! answers in. A new release is added at the end, with its changes and its contract's digest
//! (docs/api-versioning.md says how).

use super::Release;

pub const ANUBIS: Release = Release {
    name: "2026-11-04.anubis",
    contract_sha256: "ddfe623b223319b41c37bacb282ce91599aec582542132faf08555dcae65d14e",
    changes: &[],
};

/// Every release osiris serves, oldest first.
pub const RELEASES: &[Release] = &[ANUBIS];

/// The release the code answers in, and a request naming no version is served at by default.
pub const CURRENT: &Release = &ANUBIS;

/// The release named `name`, if osiris serves it.
pub fn find(name: &str) -> Option<&'static Release> {
    RELEASES.iter().find(|release| release.name == name)
}

pub fn is_known(name: &str) -> bool {
    find(name).is_some()
}
