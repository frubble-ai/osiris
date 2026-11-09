//! A 422 answers GitHub's reason, keeping its status; other errors read by their status.

use super::parse_error;
use crate::ProviderError;
use reqwest::StatusCode;
use rstest::rstest;
use serde_json::json;

#[rstest]
#[case::a_message_alone(
    json!({ "message": "Query must include 'is:issue' or 'is:pull-request'" }),
    "Query must include 'is:issue' or 'is:pull-request'"
)]
#[case::fields_and_codes(
    json!({ "message": "Validation Failed", "errors": [
        { "resource": "Issue", "field": "title", "code": "missing_field" },
        { "resource": "Label", "code": "already_exists" },
    ] }),
    "Validation Failed: Issue.title missing_field; Label already_exists"
)]
#[case::sentences(
    json!({ "message": "Validation Failed", "errors": [
        { "resource": "PullRequest", "code": "custom", "message": "No commits between main and main" },
        "Hook already exists on this repository",
    ] }),
    "Validation Failed: No commits between main and main; Hook already exists on this repository"
)]
fn answers_github_s_reason_for_a_422(#[case] body: serde_json::Value, #[case] reason: &str) {
    let error = parse_error(StatusCode::UNPROCESSABLE_ENTITY, &body.to_string());

    let ProviderError::Invalid { message, .. } = &error else {
        panic!("{error:?}");
    };
    assert_eq!(message, reason);
    assert_eq!(error.status(), Some(422));
}

#[test]
fn answers_a_422_that_isn_t_json_as_it_came() {
    let error = parse_error(StatusCode::UNPROCESSABLE_ENTITY, "  not json \n");

    let ProviderError::Invalid { message, .. } = &error else {
        panic!("{error:?}");
    };
    assert_eq!(message, "not json");
}

#[rstest]
#[case::not_found(StatusCode::NOT_FOUND, Some(404))]
#[case::forbidden(StatusCode::FORBIDDEN, Some(403))]
fn reads_other_refusals_by_their_status(#[case] status: StatusCode, #[case] kept: Option<u16>) {
    let error = parse_error(status, r#"{"message":"Not Found"}"#);

    assert!(matches!(error, ProviderError::Refused { .. }), "{error:?}");
    assert_eq!(error.status(), kept);
}

#[test]
fn reads_a_rate_limit_as_a_failure_for_now() {
    let error = parse_error(
        StatusCode::FORBIDDEN,
        r#"{"message":"API rate limit exceeded"}"#,
    );

    assert!(
        matches!(error, ProviderError::Unavailable { .. }),
        "{error:?}"
    );
}
