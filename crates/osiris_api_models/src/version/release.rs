//! A named release: its name, the digest of its contract, and the changes it made to the release
//! before.

use super::VersionChange;
use time::{Date, Month};

pub struct Release {
    /// `<date>.<name>`, such as `2026-11-04.anubis`.
    pub name: &'static str,
    /// The SHA-256 of the release's contract ([`super::contract`]). A test regenerates it from the
    /// code and fails when they differ, so a released contract can't change unnoticed.
    pub contract_sha256: &'static str,
    /// The changes this release made, oldest first; empty for the first release.
    pub changes: &'static [&'static dyn VersionChange],
}

impl Release {
    /// The day the release came out: the date its name starts with.
    pub fn released_on(&self) -> Option<Date> {
        let mut parts = self.name.get(..10)?.split('-');
        let year = parts.next()?.parse().ok()?;
        let month = Month::try_from(parts.next()?.parse::<u8>().ok()?).ok()?;
        let day = parts.next()?.parse().ok()?;
        Date::from_calendar_date(year, month, day).ok()
    }

    /// The release's god, the part of its name after the date.
    pub fn god(&self) -> &'static str {
        self.name.get(11..).unwrap_or_default()
    }
}

impl std::fmt::Debug for Release {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.name)
    }
}
