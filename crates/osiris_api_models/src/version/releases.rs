//! Every release osiris serves, oldest first. The last is the current one: the shape the code
//! answers in. A new release is added at the end as a preview, with its changes
//! (docs/api-versioning.md says how).

use super::{Release, Stability};

pub const ANUBIS: Release = Release {
    name: "2026-11-04.anubis",
    number: 1,
    // The first release is a preview while the app is its only client; it becomes stable in the
    // pull request that cuts it, before a client depends on it in production.
    stability: Stability::Preview,
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
