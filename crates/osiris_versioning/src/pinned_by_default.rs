//! The release a request or an event naming no version is pinned to by default: the newest
//! stable release, so cutting a new preview never moves clients that name no version onto it. With
//! no stable release yet, it is the newest one (today anubis, the first preview).

use crate::{CURRENT, RELEASES, Release, Stability};

/// The newest stable release of `releases`, or the newest of all when none is stable yet.
pub fn pinned_by_default_among(releases: &[Release]) -> Option<&Release> {
    releases
        .iter()
        .rev()
        .find(|release| release.stability == Stability::Stable)
        .or_else(|| releases.last())
}

/// The release `[api] pinned_version` defaults to.
pub fn pinned_by_default() -> &'static Release {
    pinned_by_default_among(RELEASES).unwrap_or(CURRENT)
}

#[cfg(test)]
mod tests;
