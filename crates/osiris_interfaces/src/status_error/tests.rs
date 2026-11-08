//! An error answer reads by its status: a rejected token, a failure for now, or a refusal that
//! keeps its status.

use super::status_error;
use crate::ProviderError;
use reqwest::StatusCode;
use rstest::rstest;

#[rstest]
#[case::rejected_token(401, "unauthorized", None)]
#[case::timeout(408, "unavailable", None)]
#[case::rate_limit(429, "unavailable", None)]
#[case::not_found(404, "refused", Some(404))]
#[case::unprocessable(422, "refused", Some(422))]
#[case::server_error(500, "unavailable", None)]
#[case::bad_gateway(502, "unavailable", None)]
fn reads_an_error_answer_by_its_status(
    #[case] status: u16,
    #[case] reading: &str,
    #[case] kept_status: Option<u16>,
) {
    let error = status_error("acme", StatusCode::from_u16(status).unwrap(), "no");

    let as_read = match reading {
        "unauthorized" => matches!(error, ProviderError::Unauthorized { .. }),
        "refused" => matches!(error, ProviderError::Refused { .. }),
        _ => matches!(error, ProviderError::Unavailable { .. }),
    };
    assert!(as_read, "{error}");
    assert_eq!(error.status(), kept_status);
}
