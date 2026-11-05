//! What goes into the auth-config operations.

use osiris_common::secret::Secret;

/// What a new auth config is made of.
pub struct NewAuthConfig {
    pub provider: String,
    pub client_id: String,
    pub client_secret: Secret,
    pub scopes: Vec<String>,
}

/// What may change on an auth config; `None` leaves a field as it is.
#[derive(Default)]
pub struct AuthConfigChanges {
    pub client_id: Option<String>,
    pub client_secret: Option<Secret>,
    pub scopes: Option<Vec<String>>,
}
