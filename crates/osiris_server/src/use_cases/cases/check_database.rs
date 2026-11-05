//! Whether the process can do its work: a cheap query (`SELECT 1`) that the database answers.

use crate::use_cases::{UseCaseResult, UseCases};

impl UseCases {
    pub async fn check_database(&self) -> UseCaseResult<bool> {
        self.run("check_database", async {
            Ok(self.context().store.ping().await)
        })
        .await
    }
}
