//! A token is reused until it is within the margin of its expiry, and dropped when refused or when
//! its app's credentials change.

use super::InstallationTokenCache;
use osiris_common::secret::{ExposeSecret, secret};
use time::{Duration, OffsetDateTime};

#[test]
fn reuses_a_token_until_it_is_within_the_margin_of_its_expiry() {
    let cache = InstallationTokenCache::default();
    let now = OffsetDateTime::now_utc();
    cache.put("ac_1", "7", secret("ghs_1"), now + Duration::minutes(60));

    let fresh = cache.get("ac_1", "7", now, Duration::minutes(5));
    let late = cache.get(
        "ac_1",
        "7",
        now + Duration::minutes(56),
        Duration::minutes(5),
    );

    assert_eq!(fresh.unwrap().expose_secret(), "ghs_1");
    assert!(late.is_none());
}

#[test]
fn drops_a_refused_token_and_every_token_of_a_changed_app() {
    let cache = InstallationTokenCache::default();
    let now = OffsetDateTime::now_utc();
    let later = now + Duration::minutes(60);
    cache.put("ac_1", "7", secret("a"), later);
    cache.put("ac_1", "8", secret("b"), later);
    cache.put("ac_2", "7", secret("c"), later);

    cache.forget("ac_1", "7");
    assert!(cache.get("ac_1", "7", now, Duration::ZERO).is_none());
    assert!(cache.get("ac_1", "8", now, Duration::ZERO).is_some());

    cache.forget_auth_config("ac_1");
    assert!(cache.get("ac_1", "8", now, Duration::ZERO).is_none());
    assert!(cache.get("ac_2", "7", now, Duration::ZERO).is_some());
}

#[test]
fn reads_a_margin_too_long_to_compute_as_no_token_instead_of_overflowing() {
    let cache = InstallationTokenCache::default();
    let now = OffsetDateTime::now_utc();
    cache.put("ac_1", "7", secret("ghs_1"), now + Duration::minutes(60));

    assert!(cache.get("ac_1", "7", now, Duration::MAX).is_none());
}
