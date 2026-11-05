//! A connection's tokens, decrypted.

use osiris_common::secret::Secret;
use time::{Duration, OffsetDateTime};

#[derive(Debug, Clone)]
pub struct TokenSet {
    pub access_token: Secret,
    pub refresh_token: Option<Secret>,
    /// None when the access token does not expire.
    pub access_token_expires_at: Option<OffsetDateTime>,
    pub refresh_token_issued_at: Option<OffsetDateTime>,
    pub scope: Option<String>,
}

impl TokenSet {
    /// The access token expires within `margin` of `now`, and a refresh token can replace it.
    pub fn needs_refresh(&self, now: OffsetDateTime, margin: Duration) -> bool {
        self.refresh_token.is_some()
            && self
                .access_token_expires_at
                .is_some_and(|expires_at| expires_at - margin <= now)
    }
}

#[cfg(test)]
mod tests {
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
}
