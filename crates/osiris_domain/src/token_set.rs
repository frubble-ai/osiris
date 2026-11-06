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
mod tests;
