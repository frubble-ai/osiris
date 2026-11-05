//! `/v1/auth-configs`.

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateAuthConfig {
    /// A provider osiris implements: `github`, `linear` or `slack`.
    pub provider: String,
    pub client_id: String,
    pub client_secret: String,
    #[serde(default)]
    pub scopes: Vec<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UpdateAuthConfig {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub scopes: Option<Vec<String>>,
}

/// An auth config, without its secret.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthConfig {
    /// `ac_…`.
    pub id: String,
    pub provider: String,
    pub client_id: String,
    pub scopes: Vec<String>,
    #[serde(with = "time::serde::rfc3339")]
    #[schema(value_type = String, format = DateTime)]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: OffsetDateTime,
}

impl From<osiris_domain::AuthConfig> for AuthConfig {
    fn from(config: osiris_domain::AuthConfig) -> Self {
        Self {
            id: config.id,
            provider: config.provider,
            client_id: config.client_id,
            scopes: config.scopes,
            created_at: config.created_at,
            updated_at: config.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Deleted {
    pub id: String,
    pub deleted: bool,
}
