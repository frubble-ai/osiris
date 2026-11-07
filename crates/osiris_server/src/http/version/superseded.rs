//! When a release was superseded and when its support ends: the next release's date, and that
//! date plus the support window. A release with no release after it is neither.

use osiris_versioning::Release;
use time::{Duration, OffsetDateTime};

/// The day `release` was superseded and its sunset, or None while it is the newest of `releases`.
pub fn superseded(
    releases: &[Release],
    release: &Release,
    support: Duration,
) -> Option<(OffsetDateTime, OffsetDateTime)> {
    let next_release = releases
        .iter()
        .skip_while(|listed| listed.name != release.name)
        .nth(1)?;
    let since = next_release.released_on()?.midnight().assume_utc();
    Some((since, since.saturating_add(support)))
}
