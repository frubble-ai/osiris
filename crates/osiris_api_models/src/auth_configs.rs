//! `/v1/auth-configs`.

use crate::secret_settings::SecretSettings;
use osiris_common::secret::Secret;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use time::OffsetDateTime;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAuthConfig {
    /// A provider osiris implements: `github`, `linear` or `slack`.
    pub provider: String,
    pub client_id: String,
    #[schema(value_type = String)]
    pub client_secret: Secret,
    #[serde(default)]
    pub scopes: Vec<String>,
    /// The provider's own settings for the app, which it validates; answered back. A GitHub app
    /// that acts as its installations names its App id here: `{"app_id": "12345"}`.
    #[serde(default)]
    #[schema(value_type = Object)]
    pub settings: Map<String, Value>,
    /// The provider's own secret settings for the app, never answered. A GitHub app that acts as
    /// its installations puts its App's private key here: `{"private_key": "-----BEGIN …"}`.
    #[schema(value_type = Option<Object>)]
    pub secret_settings: Option<SecretSettings>,
}

#[derive(Debug, Default, Deserialize, ToSchema)]
pub struct UpdateAuthConfig {
    pub client_id: Option<String>,
    #[schema(value_type = Option<String>)]
    pub client_secret: Option<Secret>,
    pub scopes: Option<Vec<String>>,
    /// Replaces the app's settings, when given.
    #[schema(value_type = Option<Object>)]
    pub settings: Option<Map<String, Value>>,
    /// Replaces the app's secret settings, when given.
    #[schema(value_type = Option<Object>)]
    pub secret_settings: Option<SecretSettings>,
}

/// An auth config, without its secret.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthConfig {
    /// `ac_…`.
    pub id: String,
    pub provider: String,
    pub client_id: String,
    pub scopes: Vec<String>,
    /// The provider's own settings for the app; an empty object when it has none.
    #[schema(value_type = Object)]
    pub settings: Value,
    /// Whether the app has secret settings (which are never answered).
    pub has_secret_settings: bool,
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
            settings: config.settings,
            has_secret_settings: config.secret_settings.is_some(),
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
