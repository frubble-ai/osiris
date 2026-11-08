//! GitHub's error answers: read by their status, except a rate limit, which GitHub may answer
//! with a 403 instead of a 429 (its primary and secondary rate limits). It is a failure for now,
//! not a refusal.

use crate::ProviderError;
use osiris_interfaces::status_error::status_error;
use reqwest::StatusCode;

pub fn parse_error(status: StatusCode, body: &str) -> ProviderError {
    if status == StatusCode::FORBIDDEN && body.to_ascii_lowercase().contains("rate limit") {
        return ProviderError::unavailable("github", format!("{status}: {body}"));
    }
    status_error("github", status, body)
}
