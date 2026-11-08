//! A client's OAuth app at a provider.

use osiris_common::secret::Secret;
use time::OffsetDateTime;

#[derive(Debug, Clone)]
pub struct AuthConfig {
    /// `ac_…`.
    pub id: String,
    /// A provider osiris implements: a row of `provider`.
    pub provider: String,
    pub client_id: String,
    pub client_secret: Secret,
    pub scopes: Vec<String>,
    /// The provider's own settings for the app, which it validates and osiris may answer (the
    /// app's id at the provider); an empty object when it has none.
    pub settings: serde_json::Value,
    /// The provider's own secret settings for the app, as a JSON object (the key the app signs
    /// with); never answered.
    pub secret_settings: Option<Secret>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
