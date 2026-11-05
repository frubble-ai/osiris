//! Delete an app no connection uses; one still in use is a conflict.

use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::Db;
use crate::use_cases::lib::database::errors::db;
use error_stack::Report;
use osiris_domain::DomainError;

const IN_USE: &str = "the auth config still has connected accounts; delete them first";

/// The foreign key from a connection to its app refused the delete: a connection was made after
/// the statement's check.
fn is_foreign_key_violation(error: &sqlx::Error) -> bool {
    error
        .as_database_error()
        .is_some_and(|error| error.code().as_deref() == Some("23503"))
}

impl Db {
    pub async fn delete_auth_config(&self, id: &str) -> UseCaseResult<()> {
        // One statement, so a connection made meanwhile cannot be deleted along with the app.
        let deleted = sqlx::query!(
            "DELETE FROM auth_config WHERE id = $1
               AND NOT EXISTS (SELECT 1 FROM connected_account WHERE auth_config_id = $1)",
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|error| {
            if is_foreign_key_violation(&error) {
                Report::new(DomainError::Conflict(IN_USE.to_owned()))
            } else {
                db(error)
            }
        })?
        .rows_affected();
        if deleted == 1 {
            return Ok(());
        }
        let exists = sqlx::query_scalar!(
            r#"SELECT EXISTS (SELECT 1 FROM auth_config WHERE id = $1) AS "exists!""#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(db)?;
        if exists {
            return Err(DomainError::Conflict(IN_USE.to_owned()).into());
        }
        Err(DomainError::NotFound("auth config").into())
    }
}
