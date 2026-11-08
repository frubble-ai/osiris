//! A trigger removed; one already gone counts as removed.

use crate::use_cases::UseCaseResult;
use crate::use_cases::lib::database::{Db, errors::db};

impl Db {
    pub async fn delete_trigger(&self, id: &str) -> UseCaseResult<()> {
        sqlx::query!("DELETE FROM trigger WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(db)?;
        Ok(())
    }
}
