//! What a provider answers for a code or a refresh.

#[derive(Debug, Clone)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: Option<String>,
    /// Seconds until the access token expires; none when it does not.
    pub expires_in: Option<i64>,
    pub scope: Option<String>,
}
