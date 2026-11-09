//! An id is taken as a number or as the text an answer gives it in, and nothing else.

use super::GithubId;
use rstest::rstest;
use serde_json::{Value, json};

#[rstest]
#[case::a_number(json!(5_820_394_923_u64))]
#[case::its_text(json!("5820394923"))]
fn takes_a_number_or_its_text(#[case] value: Value) {
    let id: GithubId = serde_json::from_value(value).unwrap();

    assert_eq!(id, GithubId(5_820_394_923));
    assert_eq!(id.to_string(), "5820394923");
}

#[rstest]
#[case::a_negative_number(json!(-1))]
#[case::a_fraction(json!(1.5))]
#[case::empty_text(json!(""))]
#[case::text_that_is_not_a_number(json!("12a"))]
#[case::a_signed_text(json!("-1"))]
#[case::too_big(json!("99999999999999999999999"))]
#[case::null(json!(null))]
fn refuses_anything_else(#[case] value: Value) {
    assert!(serde_json::from_value::<GithubId>(value).is_err());
}
