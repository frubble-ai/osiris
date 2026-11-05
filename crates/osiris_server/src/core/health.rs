//! Whether the process can do its work: a cheap query (`SELECT 1`) that the database answers.

use crate::core::Core;

impl Core {
    pub async fn database_up(&self) -> bool {
        self.store.ping().await
    }
}
