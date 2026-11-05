//! Delete an app no connection uses; one still in use is a conflict.

use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::Db;
use crate::use_cases::lib::database::errors::db;
use osiris_domain::DomainError;
impl Db {
    pub async fn delete_auth_config(&self, id: &str) -> UseCaseResult<()> {
        let in_use = sqlx::query_scalar!(r#"SELECT EXISTS (SELECT 1 FROM connected_account WHERE auth_config_id = $1) AS "in_use!""#, id)
            .fetch_one(&self.pool)
            .await
            .map_err(db)?;
        if in_use {
            return Err(DomainError::Conflict(
                "the auth config still has connected accounts; delete them first".to_owned(),
            )
            .into());
        }
        let deleted = sqlx::query!("DELETE FROM auth_config WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(db)?
            .rows_affected();
        if deleted == 0 {
            return Err(DomainError::NotFound("auth config").into());
        }
        Ok(())
    }
}
