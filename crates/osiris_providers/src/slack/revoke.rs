//! Slack ends a grant with `auth.revoke`, called with the token itself.

use super::{Slack, transformers::Status};
use crate::ProviderError;
use osiris_interfaces::call::{Request, call};
use reqwest::Method;

pub async fn revoke(
    slack: &Slack,
    http: &reqwest::Client,
    access_token: &str,
) -> Result<(), ProviderError> {
    let status: Status = call::<Status, ()>(
        slack,
        http,
        access_token,
        Request {
            method: Method::POST,
            path: "auth.revoke",
            query: &[],
            headers: &[],
            body: None,
        },
    )
    .await?;
    // Revoked now, or a token Slack no longer has.
    if status.ok || status.token_gone() {
        return Ok(());
    }
    Err(ProviderError::unavailable(
        "slack",
        format!(
            "auth.revoke failed: {}",
            status.error.as_deref().unwrap_or("unknown")
        ),
    ))
}
