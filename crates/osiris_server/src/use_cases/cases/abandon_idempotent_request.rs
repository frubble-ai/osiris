//! A keyed request failed before answering: frees its key for a retry.

use crate::use_cases::{UseCaseResult, UseCases};

pub struct AbandonIdempotentRequestArgs<'a> {
    pub key: &'a str,
}

impl UseCases {
    pub async fn abandon_idempotent_request(
        &self,
        args: AbandonIdempotentRequestArgs<'_>,
    ) -> UseCaseResult<()> {
        self.run("abandon_idempotent_request", async {
            self.context().db.abandon_idempotent(args.key).await
        })
        .await
    }
}
