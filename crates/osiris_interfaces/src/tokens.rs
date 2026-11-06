//! What a provider answers for a code or a refresh.

use osiris_common::secret::Secret;

#[derive(Debug, Clone)]
pub struct Tokens {
    pub access_token: Secret,
    pub refresh_token: Option<Secret>,
    /// Seconds until the access token expires; none when it does not.
    pub expires_in: Option<i64>,
    pub scope: Option<String>,
}
