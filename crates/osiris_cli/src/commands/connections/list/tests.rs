//! Tests of `list`.

use super::*;

#[test]
fn filters_by_what_it_was_given_naming_the_user_as_osiris_does() {
    let list = List {
        user: Some("org_1".into()),
        provider: Some("github".into()),
        status: Some("active".into()),
        after: None,
        limit: Some(5),
    };

    assert_eq!(
        list.query(),
        [
            ("user_id", "org_1".to_owned()),
            ("provider", "github".to_owned()),
            ("status", "active".to_owned()),
            ("limit", "5".to_owned()),
        ]
    );
}

#[test]
fn lists_everything_by_default() {
    let list = List {
        user: None,
        provider: None,
        status: None,
        after: None,
        limit: None,
    };

    assert!(list.query().is_empty());
}
