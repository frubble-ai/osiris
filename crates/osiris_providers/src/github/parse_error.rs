//! GitHub's error answers: read by their status, except a rate limit, which GitHub may answer
//! with a 403 instead of a 429 (its primary and secondary rate limits). It is a failure for now,
//! not a refusal. A 422 is GitHub saying why it won't take what the caller sent, so its reason is
//! answered to the caller, to change the input by.

use crate::ProviderError;
use osiris_interfaces::status_error::status_error;
use reqwest::StatusCode;
use serde::Deserialize;

pub fn parse_error(status: StatusCode, body: &str) -> ProviderError {
    if status == StatusCode::FORBIDDEN && body.to_ascii_lowercase().contains("rate limit") {
        return ProviderError::unavailable("github", format!("{status}: {body}"));
    }
    if status == StatusCode::UNPROCESSABLE_ENTITY {
        return ProviderError::invalid_with_status("github", status.as_u16(), reason(body));
    }
    status_error("github", status, body)
}

/// A 422 as GitHub writes it: a message, and what it found wrong with each field.
#[derive(Deserialize)]
struct Unprocessable {
    message: String,
    #[serde(default)]
    errors: Vec<FieldError>,
}

/// One thing GitHub found wrong: a sentence, or the field and a code naming the problem.
#[derive(Deserialize)]
#[serde(untagged)]
enum FieldError {
    Text(String),
    Field {
        message: Option<String>,
        resource: Option<String>,
        field: Option<String>,
        code: Option<String>,
    },
}

/// GitHub's message and what it found wrong, as one sentence; the body as it came when it isn't
/// GitHub's usual shape.
fn reason(body: &str) -> String {
    let Ok(answer) = serde_json::from_str::<Unprocessable>(body) else {
        return body.trim().to_owned();
    };
    let details: Vec<String> = answer
        .errors
        .into_iter()
        .filter_map(|error| match error {
            FieldError::Text(text) => Some(text),
            FieldError::Field {
                message: Some(message),
                ..
            } => Some(message),
            FieldError::Field {
                resource,
                field,
                code,
                ..
            } => {
                let place = [resource, field]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>()
                    .join(".");
                let named = [Some(place).filter(|place| !place.is_empty()), code]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>()
                    .join(" ");
                Some(named).filter(|named| !named.is_empty())
            }
        })
        .collect();
    if details.is_empty() {
        answer.message
    } else {
        format!("{}: {}", answer.message, details.join("; "))
    }
}

#[cfg(test)]
mod tests;
