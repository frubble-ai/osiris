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
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
