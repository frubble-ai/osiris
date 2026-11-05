//! The handle callers hold, and the wrapper every use case runs through: it logs the use case's
//! name when it is called and when it succeeds, with how long it took. A failure is returned as it
//! is; the caller that handles it (an HTTP route, the worker) logs it once.

use super::{UseCaseContext, UseCaseResult};
use std::{sync::Arc, time::Instant};

/// The use cases, over the context built at boot. Cloning shares the context.
#[derive(Clone)]
pub struct UseCases {
    context: Arc<UseCaseContext>,
}

impl UseCases {
    pub fn new(context: UseCaseContext) -> Self {
        Self {
            context: Arc::new(context),
        }
    }

    /// The context, for the use cases in `cases/` and for tests.
    pub(crate) fn context(&self) -> &UseCaseContext {
        &self.context
    }

    /// Runs one use case, named for the logs.
    pub(crate) async fn run<T>(
        &self,
        name: &'static str,
        body: impl Future<Output = UseCaseResult<T>>,
    ) -> UseCaseResult<T> {
        tracing::info!(use_case = name, "[use-case] {name} called");
        let started = Instant::now();
        // Boxed: use cases call use cases, and their futures nested in place would grow with
        // every level.
        let result = Box::pin(body).await;
        if result.is_ok() {
            let duration_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);
            tracing::info!(use_case = name, duration_ms, "[use-case] {name} ok");
        }
        result
    }
}
