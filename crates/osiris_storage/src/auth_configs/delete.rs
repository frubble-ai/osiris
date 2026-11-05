//! Delete an app no connection uses; one still in use is a conflict.

use crate::{
    Store,
    errors::{StorageError, StorageResult, db},
};

impl Store {
    pub async fn delete_auth_config(&self, id: &str) -> StorageResult<()> {
        let in_use = sqlx::query_scalar!(r#"SELECT EXISTS (SELECT 1 FROM connected_account WHERE auth_config_id = $1) AS "in_use!""#, id)
            .fetch_one(&self.pool)
            .await
            .map_err(db)?;
        if in_use {
            return Err(StorageError::Conflict(
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
            return Err(StorageError::NotFound("auth config").into());
        }
        Ok(())
    }
}
