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
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::an_osiris_error(
        StatusCode::CONFLICT,
        r#"{"error":{"code":"OS_06","type":"connection_not_active","message":"the connection is expired: reconnect it"}}"#,
        "OS_06 connection_not_active: the connection is expired: reconnect it"
    )]
    #[case::anything_else(
        StatusCode::BAD_GATEWAY,
        "upstream gone",
        "502 Bad Gateway: upstream gone"
    )]
    fn names_the_error_osiris_answered(
        #[case] status: StatusCode,
        #[case] body: &str,
        #[case] shown: &str,
    ) {
        assert_eq!(error_from_answer(status, body).to_string(), shown);
    }
}
