//! A request with an idempotency key arrives: run it, replay its first answer, or refuse it.

use crate::use_cases::{UseCaseResult, UseCases};
use osiris_domain::Begun;

pub struct BeginIdempotentRequestArgs<'a> {
    /// The key, scoped to the caller, the method and the path.
    pub key: &'a str,
    pub request_hash: &'a str,
}

impl UseCases {
    pub async fn begin_idempotent_request(
        &self,
        args: BeginIdempotentRequestArgs<'_>,
    ) -> UseCaseResult<Begun> {
        self.run("begin_idempotent_request", async {
            let ctx = self.context();
            let idempotency = &ctx.config.settings.idempotency;
            ctx.db
                .begin_idempotent(
                    args.key,
                    args.request_hash,
                    idempotency.retention(),
                    idempotency.purge_batch,
                )
                .await
        })
        .await
    }
}
