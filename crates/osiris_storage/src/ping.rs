//! The database answers.

use crate::Store;

impl Store {
    pub async fn ping(&self) -> bool {
        sqlx::query!("SELECT 1 AS one")
            .fetch_one(&self.pool)
            .await
            .is_ok()
    }
}
