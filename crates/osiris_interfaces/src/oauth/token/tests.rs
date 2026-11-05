//! Tests of `token`.

use super::*;
use osiris_common::secret::ExposeSecret;
use reqwest::StatusCode;
use rstest::rstest;

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

#[rstest]
#[case::error_field(
    r#"{"error":"bad_verification_code","error_description":"The code is incorrect."}"#
)]
#[case::ok_false(r#"{"ok":false,"error":"invalid_code"}"#)]
fn treats_an_error_or_ok_false_as_a_refusal_even_with_a_200(#[case] body: &str) {
    let error = parse_token_answer("test", StatusCode::OK, body).unwrap_err();
    assert!(matches!(error, ProviderError::Refused { .. }));
}

#[test]
fn never_repeats_a_body_it_cannot_read() {
    let error = parse_token_answer("test", StatusCode::BAD_GATEWAY, "secret=abc").unwrap_err();
    assert!(!error.to_string().contains("secret=abc"));
}

#[rstest]
#[case::unavailable(StatusCode::SERVICE_UNAVAILABLE, "Service Unavailable")]
#[case::server_error(StatusCode::INTERNAL_SERVER_ERROR, r#"{"error":"server_error"}"#)]
#[case::rate_limited(StatusCode::TOO_MANY_REQUESTS, r#"{"error":"slow_down"}"#)]
#[case::timeout(StatusCode::REQUEST_TIMEOUT, "")]
fn treats_a_server_error_or_rate_limit_as_temporary(
    #[case] status: StatusCode,
    #[case] body: &str,
) {
    let error = parse_token_answer("test", status, body).unwrap_err();
    assert!(matches!(error, ProviderError::Unavailable { .. }));
}

#[test]
fn treats_invalid_grant_as_permanent() {
    let error = parse_token_answer(
        "test",
        StatusCode::BAD_REQUEST,
        r#"{"error":"invalid_grant"}"#,
    )
    .unwrap_err();
    assert!(matches!(error, ProviderError::Refused { .. }));
}

#[rstest]
#[case::ok_unavailable(StatusCode::OK, "temporarily_unavailable")]
#[case::bad_request_unavailable(StatusCode::BAD_REQUEST, "temporarily_unavailable")]
#[case::ok_server_error(StatusCode::OK, "server_error")]
fn treats_an_error_code_that_says_try_again_as_temporary_whatever_the_status(
    #[case] status: StatusCode,
    #[case] code: &str,
) {
    let body = format!(r#"{{"error":"{code}"}}"#);
    let error = parse_token_answer("test", status, &body).unwrap_err();
    assert!(matches!(error, ProviderError::Unavailable { .. }));
}

#[test]
fn treats_an_error_code_that_says_the_grant_is_dead_as_permanent() {
    let dead =
        parse_token_answer("test", StatusCode::OK, r#"{"error":"invalid_grant"}"#).unwrap_err();
    assert!(matches!(dead, ProviderError::Refused { .. }));
}
