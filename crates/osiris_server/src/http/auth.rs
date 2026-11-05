//! The two keys: the API key for clients (connections, actions) and the admin key for operators
//! (providers' apps, refreshing a connection now, test events). Sent as `Authorization: Bearer <key>`, compared in constant
//! time.

use super::{error::ApiError, state::ApiState};
use axum::http::{HeaderMap, header::AUTHORIZATION};
use osiris_common::secret::ExposeSecret;
use subtle::ConstantTimeEq;

/// Which key a route asks for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Auth {
    Api,
    Admin,
}

pub fn check(state: &ApiState, headers: &HeaderMap, auth: Auth) -> Result<(), ApiError> {
    let sent = headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "));
    let expected = match auth {
        Auth::Api => state.api_key.expose_secret(),
        Auth::Admin => state.admin_key.expose_secret(),
    };
    match sent {
        Some(sent) if bool::from(sent.as_bytes().ct_eq(expected.as_bytes())) => Ok(()),
        _ => Err(ApiError::Unauthorized),
    }
}

/// Which valid key the request carries, if any.
pub fn principal(state: &ApiState, headers: &HeaderMap) -> Option<Auth> {
    [Auth::Api, Auth::Admin]
        .into_iter()
        .find(|auth| check(state, headers, *auth).is_ok())
}
