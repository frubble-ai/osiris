//! Keeps a keyed request's first answer, to replay.

use crate::use_cases::{UseCaseResult, UseCases};
use serde_json::Value;

pub struct FinishIdempotentRequestArgs<'a> {
    pub key: &'a str,
    pub status: u16,
    pub body: &'a Value,
}

impl UseCases {
    pub async fn finish_idempotent_request(
        &self,
        args: FinishIdempotentRequestArgs<'_>,
    ) -> UseCaseResult<()> {
        self.run("finish_idempotent_request", async {
            self.context()
                .db
                .finish_idempotent(args.key, args.status, args.body)
                .await
        })
        .await
    }
}
