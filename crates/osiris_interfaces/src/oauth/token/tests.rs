//! Tests of `token`.

use super::*;
use osiris_common::secret::ExposeSecret;
use reqwest::StatusCode;

#[test]
fn reads_a_token_answer_with_a_string_expiry() {
    let tokens = parse_token_answer(
        "test",
        StatusCode::OK,
        r#"{"access_token":"a","refresh_token":"r","expires_in":"3600","scope":"read"}"#,
    )
    .unwrap();
    assert_eq!(tokens.access_token.expose_secret(), "a");
    assert_eq!(
        tokens
            .refresh_token
            .as_ref()
            .map(ExposeSecret::expose_secret),
        Some("r")
    );
    assert_eq!(tokens.expires_in, Some(3600));
}

#[test]
fn treats_an_error_or_ok_false_as_a_refusal_even_with_a_200() {
    for body in [
        r#"{"error":"bad_verification_code","error_description":"The code is incorrect."}"#,
        r#"{"ok":false,"error":"invalid_code"}"#,
    ] {
        let error = parse_token_answer("test", StatusCode::OK, body).unwrap_err();
        assert!(matches!(error, ProviderError::Refused { .. }), "{body}");
    }
}

#[test]
fn never_repeats_a_body_it_cannot_read() {
    let error = parse_token_answer("test", StatusCode::BAD_GATEWAY, "secret=abc").unwrap_err();
    assert!(!error.to_string().contains("secret=abc"));
}

#[test]
fn treats_a_server_error_or_rate_limit_as_temporary_and_invalid_grant_as_permanent() {
    for (status, body) in [
        (StatusCode::SERVICE_UNAVAILABLE, "Service Unavailable"),
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            r#"{"error":"server_error"}"#,
        ),
        (StatusCode::TOO_MANY_REQUESTS, r#"{"error":"slow_down"}"#),
        (StatusCode::REQUEST_TIMEOUT, ""),
    ] {
        let error = parse_token_answer("test", status, body).unwrap_err();
        assert!(
            matches!(error, ProviderError::Unavailable { .. }),
            "{status}"
        );
    }
    let error = parse_token_answer(
        "test",
        StatusCode::BAD_REQUEST,
        r#"{"error":"invalid_grant"}"#,
    )
    .unwrap_err();
    assert!(matches!(error, ProviderError::Refused { .. }));
}

#[test]
fn treats_an_error_code_that_says_try_again_as_temporary_whatever_the_status() {
    for (status, code) in [
        (StatusCode::OK, "temporarily_unavailable"),
        (StatusCode::BAD_REQUEST, "temporarily_unavailable"),
        (StatusCode::OK, "server_error"),
    ] {
        let body = format!(r#"{{"error":"{code}"}}"#);
        let error = parse_token_answer("test", status, &body).unwrap_err();
        assert!(
            matches!(error, ProviderError::Unavailable { .. }),
            "{status} {code}"
        );
    }
    let dead =
        parse_token_answer("test", StatusCode::OK, r#"{"error":"invalid_grant"}"#).unwrap_err();
    assert!(matches!(dead, ProviderError::Refused { .. }));
}
