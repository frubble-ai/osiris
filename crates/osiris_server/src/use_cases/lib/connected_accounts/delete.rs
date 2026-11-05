//! Deletes a connection and returns the name of its pending refresh task, so the caller can
//! cancel it.

use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::Db;
use crate::use_cases::lib::database::errors::db;
use osiris_domain::DomainError;
impl Db {
    pub async fn delete_connected_account(&self, id: &str) -> UseCaseResult<Option<String>> {
        let deleted = sqlx::query_scalar!(
            "DELETE FROM connected_account WHERE id = $1 RETURNING refresh_task",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(db)?
        .ok_or(DomainError::NotFound("connected account"))?;
        Ok(deleted)
    }
}
