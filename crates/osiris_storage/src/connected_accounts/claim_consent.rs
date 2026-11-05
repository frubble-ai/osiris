//! Take a consent `state` the provider sent back: cleared as it is read, so it works once.

use super::Claimed;
use crate::{
    Store, Tenant,
    errors::{StorageResult, db},
    rows::decrypt,
};

impl Store {
    pub async fn claim_consent(&self, state: &str) -> StorageResult<Option<Claimed>> {
        let claimed = sqlx::query!(
            r#"UPDATE connected_account a SET oauth_state = NULL, oauth_state_expires_at = NULL, pkce_verifier = NULL, updated_at = now()
               FROM connected_account old
               WHERE a.id = old.id AND a.oauth_state = $1 AND a.status = 'pending'
               RETURNING a.id, a.user_id, old.pkce_verifier, old.oauth_state_expires_at < now() AS "expired?""#,
            state,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(db)?;
        let Some(claimed) = claimed else {
            return Ok(None);
        };
        let cipher = self.cipher_for(&Tenant::user(&claimed.user_id)).await?;
        Ok(Some(Claimed {
            account: self.connected_account(&claimed.id).await?,
            pkce_verifier: claimed
                .pkce_verifier
                .as_deref()
                .map(|value| decrypt(&cipher, value))
                .transpose()?,
            expired: claimed.expired.unwrap_or(true),
        }))
    }
}
