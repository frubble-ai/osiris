//! API versions, as Stripe's: a date and a named release. A client sends `Osiris-Version`; every
//! answer says the version it was served at. Breaking changes only come with a new release, named
//! after the Egyptian pantheon in alphabetical order: anubis, bastet, …

/// The version a request is served at when it sends none.
pub const CURRENT: &str = "2026-11-04.anubis";

/// Every version osiris serves, oldest first.
pub const VERSIONS: &[&str] = &[CURRENT];

pub const HEADER: &str = "osiris-version";

pub fn is_known(version: &str) -> bool {
    VERSIONS.contains(&version)
}
