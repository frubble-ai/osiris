//! Authenticates the tasks Cloud Tasks sends: each carries a Google-signed OIDC token, minted for
//! the configured service account with the worker's URL as its audience. The token's signature is
//! checked against Google's published keys, then its issuer, audience, expiry and service account.
//! The keys are cached for `keys_ttl`. A token naming a key not in the cache fetches them again, but
//! at most once per `refetch_interval`, and one fetch at a time, so a caller sending made-up key ids
//! can't make osiris call Google on every request.

use async_trait::async_trait;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header, jwk::JwkSet};
use osiris_tasks::{AuthError, TaskAuthenticator};
use serde::Deserialize;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, RwLock};

/// Where Google publishes the keys that sign its OIDC tokens.
pub const GOOGLE_KEYS_URL: &str = "https://www.googleapis.com/oauth2/v3/certs";

/// The issuers Google's OIDC tokens carry.
pub const GOOGLE_ISSUERS: [&str; 2] = ["https://accounts.google.com", "accounts.google.com"];

#[derive(Debug, Clone)]
pub struct OidcConfig {
    pub keys_url: String,
    /// The audience tasks are minted with: the worker's URL.
    pub audience: String,
    /// The service account tasks are minted for.
    pub service_account_email: String,
    pub keys_ttl: Duration,
    /// The shortest time between two fetches of the keys for a key id not in the cache.
    pub refetch_interval: Duration,
}

pub struct OidcAuthenticator {
    config: OidcConfig,
    http: reqwest::Client,
    keys: RwLock<Option<(JwkSet, Instant)>>,
    /// Held while the keys are fetched, so concurrent callers wait for one fetch.
    fetching: Mutex<()>,
}

#[derive(Deserialize)]
struct Claims {
    email: Option<String>,
    email_verified: Option<bool>,
}

impl OidcAuthenticator {
    pub fn new(config: OidcConfig, http: reqwest::Client) -> Self {
        Self {
            config,
            http,
            keys: RwLock::new(None),
            fetching: Mutex::new(()),
        }
    }

    async fn fetch_keys(&self) -> Result<JwkSet, AuthError> {
        let keys: JwkSet = self
            .http
            .get(&self.config.keys_url)
            .send()
            .await
            .and_then(reqwest::Response::error_for_status)
            .map_err(|error| AuthError::new(format!("fetching Google's keys failed: {error}")))?
            .json()
            .await
            .map_err(|error| AuthError::new(format!("reading Google's keys failed: {error}")))?;
        *self.keys.write().await = Some((keys.clone(), Instant::now()));
        Ok(keys)
    }

    /// The key `kid`, from the cache when it is fresh and holds it, otherwise from a fetch.
    async fn key(&self, kid: &str) -> Result<DecodingKey, AuthError> {
        if let Some(key) = self.cached(kid).await? {
            return Ok(key);
        }
        let _fetching = self.fetching.lock().await;
        // Another caller may have fetched while this one waited.
        if let Some(key) = self.cached(kid).await? {
            return Ok(key);
        }
        let keys = self.fetch_keys().await?;
        let jwk = keys
            .find(kid)
            .ok_or_else(|| AuthError::new("the token is signed with an unknown key"))?;
        DecodingKey::from_jwk(jwk).map_err(|error| AuthError::new(error.to_string()))
    }

    /// The key `kid` from the cache, if it is fresh and holds it. An error when the keys were
    /// fetched less than `refetch_interval` ago and still lack `kid`: fetching again would not
    /// help, and must not be something a caller can force.
    async fn cached(&self, kid: &str) -> Result<Option<DecodingKey>, AuthError> {
        let Some((keys, fetched)) = self.keys.read().await.clone() else {
            return Ok(None);
        };
        let age = fetched.elapsed();
        match keys.find(kid) {
            Some(jwk) if age < self.config.keys_ttl => DecodingKey::from_jwk(jwk)
                .map(Some)
                .map_err(|error| AuthError::new(error.to_string())),
            None if age < self.config.refetch_interval => {
                Err(AuthError::new("the token is signed with an unknown key"))
            }
            _ => Ok(None),
        }
    }
}

#[async_trait]
impl TaskAuthenticator for OidcAuthenticator {
    async fn authenticate(&self, authorization: Option<&str>) -> Result<(), AuthError> {
        let token = authorization
            .and_then(|value| value.strip_prefix("Bearer "))
            .ok_or_else(|| AuthError::new("no bearer token"))?;
        let header = decode_header(token).map_err(|error| AuthError::new(error.to_string()))?;
        let kid = header
            .kid
            .ok_or_else(|| AuthError::new("the token names no key"))?;
        let key = self.key(&kid).await?;
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[&self.config.audience]);
        validation.set_issuer(&GOOGLE_ISSUERS);
        let claims = decode::<Claims>(token, &key, &validation)
            .map_err(|error| AuthError::new(error.to_string()))?
            .claims;
        if claims.email_verified != Some(true)
            || claims.email.as_deref() != Some(self.config.service_account_email.as_str())
        {
            return Err(AuthError::new(
                "the token is not for the configured service account",
            ));
        }
        Ok(())
    }
}
