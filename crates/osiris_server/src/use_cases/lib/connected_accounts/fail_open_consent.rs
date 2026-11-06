//! Marks a consent as failed if it is still open, because nobody finished it in time. Returns
//! false when the consent was no longer open.

use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::Db;
use crate::use_cases::lib::database::errors::db;
impl Db {
    pub async fn fail_open_consent(&self, id: &str) -> UseCaseResult<bool> {
        let failed = sqlx::query!(
            "UPDATE connected_account SET status = 'failed', oauth_state = NULL, oauth_state_expires_at = NULL,
                 pkce_verifier = NULL, last_refresh_error = 'the consent was never finished', updated_at = now()
             WHERE id = $1 AND status = 'pending'",
            id,
        )
        .execute(&self.pool)
        .await
        .map_err(db)?
        .rows_affected();
        Ok(failed == 1)
    }
}
