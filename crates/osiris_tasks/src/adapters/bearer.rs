//! Authenticates tasks by a shared bearer token: the local scheduler sends it, and tests use it.
//! The token must be at least 32 characters, and is compared in constant time.

use crate::{AuthError, TaskAuthenticator};
use async_trait::async_trait;
use subtle::ConstantTimeEq;

/// The shortest token accepted, so it can't be guessed.
pub const MIN_TOKEN_LENGTH: usize = 32;

pub struct BearerAuthenticator {
    token: String,
}

impl BearerAuthenticator {
    pub fn new(token: String) -> Result<Self, AuthError> {
        if token.len() < MIN_TOKEN_LENGTH {
            return Err(AuthError::new(format!(
                "the worker token must be at least {MIN_TOKEN_LENGTH} characters"
            )));
        }
        Ok(Self { token })
    }
}

#[async_trait]
impl TaskAuthenticator for BearerAuthenticator {
    async fn authenticate(&self, authorization: Option<&str>) -> Result<(), AuthError> {
        let sent = authorization
            .and_then(|value| value.strip_prefix("Bearer "))
            .unwrap_or_default();
        if bool::from(sent.as_bytes().ct_eq(self.token.as_bytes())) {
            Ok(())
        } else {
            Err(AuthError::new("wrong or missing bearer token"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOKEN: &str = "a-worker-token-that-is-long-enough!";

    #[tokio::test]
    async fn accepts_only_the_token() {
        let auth = BearerAuthenticator::new(TOKEN.into()).unwrap();
        assert!(
            auth.authenticate(Some(&format!("Bearer {TOKEN}")))
                .await
                .is_ok()
        );
        assert!(auth.authenticate(Some("Bearer wrong")).await.is_err());
        assert!(auth.authenticate(Some(TOKEN)).await.is_err());
        assert!(auth.authenticate(None).await.is_err());
    }

    #[test]
    fn refuses_a_short_token() {
        assert!(BearerAuthenticator::new("short".into()).is_err());
    }
}
