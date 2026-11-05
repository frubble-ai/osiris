//! Tests of `token_set`.

use super::*;
use osiris_common::secret::secret;

fn tokens(expires_in: Option<i64>, refreshable: bool) -> TokenSet {
    let now = OffsetDateTime::UNIX_EPOCH;
    TokenSet {
        access_token: secret("a"),
        refresh_token: refreshable.then(|| secret("r")),
        access_token_expires_at: expires_in.map(|seconds| now + Duration::seconds(seconds)),
        refresh_token_issued_at: Some(now),
        scope: None,
    }
}

#[test]
fn refreshes_only_a_refreshable_token_inside_the_margin() {
    let now = OffsetDateTime::UNIX_EPOCH;
    let margin = Duration::minutes(5);
    assert!(tokens(Some(60), true).needs_refresh(now, margin));
    assert!(!tokens(Some(3600), true).needs_refresh(now, margin));
    assert!(!tokens(Some(60), false).needs_refresh(now, margin));
    assert!(!tokens(None, true).needs_refresh(now, margin));
}
