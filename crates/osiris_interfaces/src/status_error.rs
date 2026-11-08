//! An error answer read by its status alone, the way most providers' errors read.

use crate::ProviderError;
use reqwest::StatusCode;

/// 401 is a rejected token, a timeout (408) or a rate limit (429) a failure for now, another 4xx a
/// refusal, and the rest a failure.
pub fn status_error(provider: &'static str, status: StatusCode, body: &str) -> ProviderError {
    let message = format!("{status}: {body}");
    match status {
        StatusCode::UNAUTHORIZED => ProviderError::unauthorized(provider, message),
        StatusCode::REQUEST_TIMEOUT | StatusCode::TOO_MANY_REQUESTS => {
            ProviderError::unavailable(provider, message)
        }
        status if status.is_client_error() => {
            ProviderError::refused_with_status(provider, status.as_u16(), message)
        }
        _ => ProviderError::unavailable(provider, message),
    }
}

#[cfg(test)]
mod tests;
