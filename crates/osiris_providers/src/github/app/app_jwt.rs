//! The JWT that identifies our GitHub App to GitHub: signed with the app's private key (RS256),
//! issued by the app's id. GitHub refuses one that lives over ten minutes, so it lives nine, and
//! it is issued a minute in the past so a clock slightly ahead of GitHub's is still accepted.

use super::AppCredentials;
use crate::ProviderError;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

/// How long the JWT lives: GitHub's ten-minute limit, less a minute.
const LIFETIME_SECONDS: u64 = 9 * 60;
/// How far in the past it is issued, for a clock slightly ahead of GitHub's.
const CLOCK_SKEW_SECONDS: u64 = 60;

#[derive(Serialize)]
struct Claims<'a> {
    iss: &'a str,
    iat: u64,
    exp: u64,
}

/// A JWT for the app, issued a minute ago (for clock skew) and valid for nine minutes from then.
pub fn app_jwt(credentials: &AppCredentials) -> Result<String, ProviderError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| ProviderError::unavailable("github", error))?
        .as_secs();
    let issued_at = now.saturating_sub(CLOCK_SKEW_SECONDS);
    let key = EncodingKey::from_rsa_pem(credentials.private_key.as_bytes()).map_err(|_| {
        ProviderError::invalid(
            "github",
            "the GitHub App's private key is not an RSA key in PEM",
        )
    })?;
    encode(
        &Header::new(Algorithm::RS256),
        &Claims {
            iss: &credentials.app_id,
            iat: issued_at,
            exp: issued_at + LIFETIME_SECONDS,
        },
        &key,
    )
    .map_err(|error| ProviderError::invalid("github", format!("signing the app's JWT: {error}")))
}
