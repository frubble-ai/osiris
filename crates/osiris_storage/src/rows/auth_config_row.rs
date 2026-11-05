//! A row of `auth_config`.

use super::decrypt;
use crate::errors::StorageResult;
use osiris_common::{crypto::Cipher, secret::secret};
use osiris_domain::AuthConfig;
use time::OffsetDateTime;

pub struct AuthConfigRow {
    pub id: String,
    pub provider: String,
    pub client_id: String,
    pub client_secret: Vec<u8>,
    pub scopes: Vec<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl AuthConfigRow {
    pub fn decrypt(self, cipher: &Cipher) -> StorageResult<AuthConfig> {
        Ok(AuthConfig {
            client_secret: secret(decrypt(cipher, &self.client_secret)?),
            id: self.id,
            provider: self.provider,
            client_id: self.client_id,
            scopes: self.scopes,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}
