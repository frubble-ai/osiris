//! The database answers.

use crate::use_cases::lib::database::Db;
impl Db {
    pub async fn ping(&self) -> bool {
        sqlx::query!("SELECT 1 AS one")
            .fetch_one(&self.pool)
            .await
            .is_ok()
    }
}
