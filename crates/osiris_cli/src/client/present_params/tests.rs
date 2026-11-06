//! Tests of `present_params`.

use super::*;

#[test]
fn sends_only_the_parameters_given_and_in_order() {
    let params = present_params([
        ("user_id", Some("org_1".to_owned())),
        ("status", None),
        ("limit", Some("5".to_owned())),
    ]);

    assert_eq!(
        params,
        [("user_id", "org_1".to_owned()), ("limit", "5".to_owned())]
    );
}

#[test]
fn sends_nothing_when_nothing_is_given() {
    assert!(present_params([("after", None), ("limit", None)]).is_empty());
}
