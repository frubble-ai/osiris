//! An error answer as the error the CLI prints: osiris's code, type and message when the body is
//! one of its errors, else the status and the body.

use osiris_api_models::errors::ErrorBody;
use reqwest::StatusCode;

pub fn error_from_answer(status: StatusCode, text: &str) -> anyhow::Error {
    match serde_json::from_str::<ErrorBody>(text) {
        Ok(error) => anyhow::anyhow!(
            "{} {}: {}",
            error.error.code,
            error.error.error_type,
            error.error.message
        ),
        Err(_) => anyhow::anyhow!("{status}: {text}"),
    }
}

#[cfg(test)]
mod tests;
