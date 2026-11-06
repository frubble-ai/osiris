//! The trait the worker uses to check who sent a task. Every scheduler has one, so `/tasks` never
//! runs a task from a caller it can't authenticate.

use crate::AuthError;
use async_trait::async_trait;

#[async_trait]
pub trait TaskAuthenticator: Send + Sync {
    /// Checks the request's `Authorization` header (`None` when absent).
    async fn authenticate(&self, authorization: Option<&str>) -> Result<(), AuthError>;
}
