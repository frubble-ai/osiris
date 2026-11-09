//! Triggers, of one connection or of all, in batches (cursor pagination): `after` gives the next
//! batch, `before` the previous one.

use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::{Db, Tenant, errors::db, rows::TriggerRow};
use error_stack::Report;
use osiris_domain::{Cursor, DomainError, Page, Trigger};

impl Db {
    async fn require_trigger_cursor(&self, cursor: &Cursor) -> UseCaseResult<()> {
        let Some(id) = cursor.after.as_deref().or(cursor.before.as_deref()) else {
            return Ok(());
        };
        let found = sqlx::query_scalar!("SELECT id FROM trigger WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
            .map_err(db)?;
        if found.is_none() {
            return Err(Report::new(DomainError::Invalid(
                "the cursor names a record that no longer exists".to_owned(),
            )));
        }
        Ok(())
    }

    /// The triggers of `connected_account_id`, or of every connection when it is none.
    pub async fn list_triggers(
        &self,
        connected_account_id: Option<&str>,
        cursor: &Cursor,
    ) -> UseCaseResult<Page<Trigger>> {
        let cipher = self.cipher_for(&Tenant::Osiris).await?;
        self.require_trigger_cursor(cursor).await?;
        // One static query per direction, so `(created_at, id)` serves the order.
        let found = match cursor.before.as_deref() {
            None => sqlx::query_as!(
                TriggerRow,
                "SELECT id, connected_account_id, provider, kind, config, secret, status, subscription_id, checkpoint, last_error, poll_task, poll_task_at, created_at, updated_at FROM trigger
             WHERE ($1::text IS NULL OR connected_account_id = $1)
               AND ($2::text IS NULL OR (created_at, id) > (SELECT created_at, id FROM trigger WHERE id = $2))
             ORDER BY created_at ASC, id ASC
             LIMIT $3",
                connected_account_id,
                cursor.after.as_deref(),
                cursor.limit() + 1,
            )
            .fetch_all(&self.pool)
            .await,
            Some(before) => sqlx::query_as!(
                TriggerRow,
                "SELECT id, connected_account_id, provider, kind, config, secret, status, subscription_id, checkpoint, last_error, poll_task, poll_task_at, created_at, updated_at FROM trigger
             WHERE ($1::text IS NULL OR connected_account_id = $1)
               AND (created_at, id) < (SELECT created_at, id FROM trigger WHERE id = $2)
             ORDER BY created_at DESC, id DESC
             LIMIT $3",
                connected_account_id,
                before,
                cursor.limit() + 1,
            )
            .fetch_all(&self.pool)
            .await,
        }
        .map_err(db)?
        .into_iter()
        .map(|row| row.decrypt(&cipher))
        .collect::<UseCaseResult<Vec<_>>>()?;
        Ok(Page::from_overfetch(found, cursor, |trigger| {
            trigger.id.clone()
        }))
    }
}
