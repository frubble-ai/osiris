//! Connections, filtered, in batches (cursor pagination): `after` gives the next batch, `before`
//! the previous one.

use super::AccountFilter;
use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::Db;
use crate::use_cases::lib::database::Tenant;
use crate::use_cases::lib::database::errors::db;
use crate::use_cases::lib::database::rows::AccountRow;
use error_stack::Report;
use osiris_domain::Cursor;
use osiris_domain::DomainError;
use osiris_domain::Page;
use osiris_domain::{ConnectedAccount, ConnectionStatus};

impl Db {
    async fn require_connected_account_cursor(&self, cursor: &Cursor) -> UseCaseResult<()> {
        let Some(id) = cursor.after.as_deref().or(cursor.before.as_deref()) else {
            return Ok(());
        };
        let found = sqlx::query_scalar!("SELECT id FROM connected_account WHERE id = $1", id)
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

    pub async fn list_connected_accounts(
        &self,
        filter: &AccountFilter,
        cursor: &Cursor,
    ) -> UseCaseResult<Page<ConnectedAccount>> {
        self.require_connected_account_cursor(cursor).await?;
        // One static query per direction, so `(created_at, id)` serves the order.
        let found = match cursor.before.as_deref() {
            None => sqlx::query_as!(
                AccountRow,
                "SELECT id, auth_config_id, provider, user_id, alias, status, callback_url, access_token, refresh_token, access_token_expires_at, refresh_token_issued_at, scope, last_refresh_error, refresh_failures, token_generation, refresh_task, refresh_task_at, created_at, updated_at FROM connected_account
             WHERE ($1::text IS NULL OR user_id = $1)
               AND ($2::text IS NULL OR provider = $2)
               AND ($3::text IS NULL OR auth_config_id = $3)
               AND ($4::text IS NULL OR alias = $4)
               AND ($5::text IS NULL OR status = $5)
               AND ($6::text IS NULL OR (created_at, id) > (SELECT created_at, id FROM connected_account WHERE id = $6))
             ORDER BY created_at ASC, id ASC
             LIMIT $7",
            filter.user_id.as_deref(),
            filter.provider.as_deref(),
            filter.auth_config_id.as_deref(),
            filter.alias.as_deref(),
            filter.status.map(ConnectionStatus::as_str),
                cursor.after.as_deref(),
                cursor.limit() + 1,
            )
            .fetch_all(&self.pool)
            .await,
            Some(before) => sqlx::query_as!(
                AccountRow,
                "SELECT id, auth_config_id, provider, user_id, alias, status, callback_url, access_token, refresh_token, access_token_expires_at, refresh_token_issued_at, scope, last_refresh_error, refresh_failures, token_generation, refresh_task, refresh_task_at, created_at, updated_at FROM connected_account
             WHERE ($1::text IS NULL OR user_id = $1)
               AND ($2::text IS NULL OR provider = $2)
               AND ($3::text IS NULL OR auth_config_id = $3)
               AND ($4::text IS NULL OR alias = $4)
               AND ($5::text IS NULL OR status = $5)
               AND (created_at, id) < (SELECT created_at, id FROM connected_account WHERE id = $6)
             ORDER BY created_at DESC, id DESC
             LIMIT $7",
            filter.user_id.as_deref(),
            filter.provider.as_deref(),
            filter.auth_config_id.as_deref(),
            filter.alias.as_deref(),
            filter.status.map(ConnectionStatus::as_str),
                before,
                cursor.limit() + 1,
            )
            .fetch_all(&self.pool)
            .await,
        }
        .map_err(db)?;
        let tenants: Vec<Tenant> = found.iter().map(|row| Tenant::user(&row.user_id)).collect();
        self.load_ciphers(&tenants).await?;
        let mut accounts = Vec::with_capacity(found.len());
        for row in found {
            accounts.push(self.decrypt_account(row).await?);
        }
        Ok(Page::from_overfetch(accounts, cursor, |account| {
            account.id.clone()
        }))
    }
}
